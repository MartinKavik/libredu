use std::time::{Duration, Instant};
use actix::*;
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
//use crate::chat_server::*;
use crate::chat_server;
use crate::request::Request;
use actix_web::web::{Path, Data};
use actix_session::Session;
use actix_redis::{Command, RedisActor};
use std::collections::HashMap;

/// How often heartbeat pings are sent
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(5);
/// How long before lack of client response causes a timeout
const CLIENT_TIMEOUT: Duration = Duration::from_secs(10);

/// Entry point for our route
pub async fn chat_route(
    req: HttpRequest,
    path: Path<i32>,
    session: Session,
    stream: web::Payload,
    redis: Data<Addr<RedisActor>>,
    srv: web::Data<Addr<chat_server::ChatServer>>,
) -> Result<HttpResponse, Error> {
    let pth = path.into_inner();
    let school = req.school(pth).unwrap();
    if school.manager.unwrap()==req.user(&session).unwrap().id || req.user(&session).unwrap().is_admin.unwrap(){
        let get_generates = redis.send(Command(resp_array!["LRANGE", "generates"].append(vec!["0".to_string(),"-1".to_string()])));
        match get_generates.await{
            Ok(generates)=>{
                use redis_async::resp::FromResp;
                let gen_vec: Result<Option<Vec<String>>, _> = generates.and_then(|value| FromResp::from_resp(value).map_err(actix_redis::Error::Redis));
                match gen_vec{
                    Ok(_gen)=>{
                        match _gen{
                            Some(gen)=>{
                                if gen.iter().clone().any(|prms| &prms[1..].to_string() == &school.code.to_string()){
                                    let index = gen.iter().clone().enumerate().find(|prms| &prms.1[1..].to_string() == &school.code.to_string());
                                    match index{
                                        Some(i)=>{
                                            if i.0 == 0{
                                                ws::start(
                                                    WsChatSession {
                                                        id: 0,
                                                        hb: Instant::now(),
                                                        room: pth.to_owned().to_string(),
                                                        name: None,
                                                        addr: srv.get_ref().clone(),
                                                    },
                                                    &req,
                                                    stream,
                                                )
                                            }
                                            else{
                                                use std::fs::File;
                                                let f = File::open("../ulakgt/generates/".to_owned()+&school.code.to_string()).unwrap();
                                                let mut rslt = HashMap::new();
                                                rslt.insert("wait".to_string(), i.0*60);
                                                serde_json::to_writer(f, &rslt).unwrap();
                                                ws::start(
                                                    WsChatSession {
                                                        id: 0,
                                                        hb: Instant::now(),
                                                        room: pth.to_owned().to_string(),
                                                        name: None,
                                                        addr: srv.get_ref().clone(),
                                                    },
                                                    &req,
                                                    stream,
                                                )
                                            }
                                        }
                                        None=>{
                                            Ok(HttpResponse::Ok().content_type("application/json").json("Verileriniz bulunamadı"))
                                        }
                                    }

                                }
                                else{
                                    Ok(HttpResponse::Ok().content_type("application/json").json("Verileriniz kayıtlı değil"))
                                }
                            },
                            None=>{
                                Ok(HttpResponse::Ok().content_type("application/json").json("Kayıtlı veri yok"))

                            }
                        }
                    },
                    Err(_e)=>{
                        Ok(HttpResponse::InternalServerError().json("Veriler hatalı"))
                    }
                }

            },
            Err(_e)=>{
                Ok(HttpResponse::InternalServerError().json("Redis hatası"))
            }
        }
    }

    else{
        Ok(HttpResponse::Ok().content_type("application/json").json("Yetkiniz yok"))
    }
}

struct WsChatSession {
    /// unique session id
    id: usize,
    /// Client must send ping at least once per 10 seconds (CLIENT_TIMEOUT),
    /// otherwise we drop connection.
    hb: Instant,
    /// joined room
    room: String,
    /// peer name
    name: Option<String>,
    /// Chat server
    addr: Addr<chat_server::ChatServer>,
}

impl Actor for WsChatSession {
    type Context = ws::WebsocketContext<Self>;

    /// Method is called on actor start.
    /// We register ws session with ChatServer
    fn started(&mut self, ctx: &mut Self::Context) {
        // we'll start heartbeat process on session start.
        self.hb(ctx);

        // register self in chat server. `AsyncContext::wait` register
        // future within context, but context waits until this future resolves
        // before processing any other events.
        // HttpContext::state() is instance of WsChatSessionState, state is shared
        // across all routes within application
        let addr = ctx.address();
        self.addr
            .send(chat_server::Connect {
                addr: addr.recipient(),
            })
            .into_actor(self)
            .then(|res, act, ctx| {
                match res {
                    Ok(res) => act.id = res,
                    // something is wrong with chat server
                    _ => ctx.stop(),
                }
                fut::ready(())
            })
            .wait(ctx);
    }

    fn stopping(&mut self, _: &mut Self::Context) -> Running {
        // notify chat server
        self.addr.do_send(chat_server::Disconnect { id: self.id });
        Running::Stop
    }
}

/// Handle messages from chat server, we simply send it to peer websocket
impl Handler<chat_server::Message> for WsChatSession {
    type Result = ();

    fn handle(&mut self, msg: chat_server::Message, ctx: &mut Self::Context) {
        ctx.text(msg.0);
    }
}

/// WebSocket message handler
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for WsChatSession {
    fn handle(
        &mut self,
        msg: Result<ws::Message, ws::ProtocolError>,
        ctx: &mut Self::Context,
    ) {
        let msg = match msg {
            Err(_) => {
                ctx.stop();
                return;
            }
            Ok(msg) => msg,
        };

        println!("WEBSOCKET MESSAGE: {:?}", msg);
        match msg {
            ws::Message::Ping(msg) => {
                self.hb = Instant::now();
                ctx.pong(&msg);
            }
            ws::Message::Pong(_) => {
                use std::fs::File;
                use std::io::Read;
                let f = File::open("../ulakgt/generates/".to_owned()+&self.room);
                let mut contents = String::new();
                f.unwrap().read_to_string(&mut contents).unwrap();
                ctx.text(contents.to_owned());
                self.hb = Instant::now();
            }
            ws::Message::Text(text) => {
                ctx.text(&text);
                // we check for /sss type of messages
            }
            ws::Message::Binary(_) => println!("Unexpected binary"),
            ws::Message::Close(_) => {
                ctx.stop();
            }
            ws::Message::Continuation(_) => {
                ctx.stop();
            }
            ws::Message::Nop => (),
        }
    }
}

impl WsChatSession {
    /// helper method that sends ping to client every second.
    ///
    /// also this method checks heartbeats from client
    fn hb(&self, ctx: &mut ws::WebsocketContext<Self>) {
        ctx.run_interval(HEARTBEAT_INTERVAL, |act, ctx| {
            // check client heartbeats
            if Instant::now().duration_since(act.hb) > CLIENT_TIMEOUT {
                // heartbeat timed out
                println!("Websocket Client heartbeat failed, disconnecting!");

                // notify chat server
                act.addr.do_send(chat_server::Disconnect { id: act.id });

                // stop actor
                ctx.stop();

                // don't try to send a ping
                return;
            }

            ctx.ping(b"");
        });
    }
}