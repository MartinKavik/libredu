table! {
    activities (id) {
        id -> Int4,
        subject -> Nullable<Int4>,
        teacher -> Nullable<Int4>,
        hour -> Nullable<Int2>,
        class -> Int4,
        split -> Bool,
        splitted -> Nullable<Int2>,
        placed -> Bool,
        day -> Nullable<Int2>,
        hrs -> Nullable<Int2>,
        act_id -> Nullable<Int4>,
    }
}

table! {
    auth (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
    }
}

table! {
    city (pk) {
        pk -> Int4,
        name -> Varchar,
    }
}

table! {
    class_available (class_id, day) {
        class_id -> Int4,
        day -> Int4,
        hours -> Array<Bool>,
    }
}

table! {
    class_subjects (class, subject) {
        class -> Int4,
        subject -> Int4,
    }
}

table! {
    class_timetable (id) {
        id -> Int4,
        class_id -> Int4,
        day_id -> Int4,
        hour -> Int2,
        activities -> Int4,
    }
}

table! {
    classes (id) {
        id -> Int4,
        kademe -> Int2,
        sube -> Varchar,
        school -> Int4,
        teacher -> Nullable<Int4>,
    }
}

table! {
    content_type (id) {
        id -> Int4,
        app_label -> Varchar,
        model -> Varchar,
    }
}

table! {
    days (id) {
        id -> Int4,
        name -> Varchar,
    }
}

table! {
    hours (id) {
        id -> Int4,
    }
}

table! {
    post (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        only_teacher -> Nullable<Bool>,
        body -> Text,
        pub_date -> Nullable<Timestamptz>,
        sender -> Int4,
        school -> Nullable<Int4>,
    }
}

table! {
    school (code) {
        code -> Int4,
        name -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        pansiyon -> Nullable<Bool>,
        dersane -> Nullable<Bool>,
        is_active -> Nullable<Bool>,
        manager -> Nullable<Int4>,
        city -> Nullable<Int4>,
        town -> Nullable<Int4>,
        school_type -> Nullable<Int4>,
        hour -> Int2,
    }
}

table! {
    school_grade (school_type, grade) {
        school_type -> Int4,
        grade -> Int2,
    }
}

table! {
    school_menu (id) {
        title -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
        school_type -> Nullable<Int4>,
        id -> Int4,
    }
}

table! {
    school_subjects (id) {
        school -> Int4,
        subject -> Int4,
        id -> Int4,
    }
}

table! {
    school_time (id) {
        id -> Int4,
        school -> Nullable<Int4>,
        day -> Nullable<Int4>,
        start -> Nullable<Time>,
        finish -> Nullable<Time>,
        hour -> Nullable<Int4>,
    }
}

table! {
    school_type (id) {
        name -> Varchar,
        id -> Int4,
    }
}

table! {
    school_users (school_id, user_id) {
        user_id -> Int4,
        school_id -> Int4,
        auth -> Nullable<Int4>,
    }
}

table! {
    session (id) {
        id -> Int4,
        session_key -> Varchar,
        session_data -> Text,
        expire_date -> Timestamptz,
    }
}

table! {
    subjects (id) {
        id -> Int4,
        name -> Nullable<Varchar>,
        kademe -> Nullable<Int2>,
        school_type -> Nullable<Int4>,
        optional -> Nullable<Bool>,
    }
}

table! {
    teacher_available (user_id, school_id, day) {
        user_id -> Int4,
        school_id -> Int4,
        day -> Int4,
        hours -> Array<Bool>,
    }
}

table! {
    teachers_menu (id) {
        id -> Int4,
        title -> Nullable<Varchar>,
        link -> Nullable<Varchar>,
    }
}

table! {
    town (pk) {
        pk -> Int4,
        name -> Varchar,
        city -> Nullable<Int4>,
    }
}

table! {
    users (id) {
        id -> Int4,
        first_name -> Nullable<Varchar>,
        last_name -> Nullable<Varchar>,
        username -> Nullable<Varchar>,
        email -> Nullable<Varchar>,
        password -> Nullable<Varchar>,
        date_join -> Nullable<Timestamp>,
        last_login -> Nullable<Timestamp>,
        is_active -> Nullable<Bool>,
        is_staff -> Nullable<Bool>,
        is_admin -> Nullable<Bool>,
        tel -> Nullable<Varchar>,
        gender -> Nullable<Varchar>,
        img -> Nullable<Varchar>,
    }
}

joinable!(activities -> classes (class));
joinable!(activities -> subjects (subject));
joinable!(activities -> users (teacher));
joinable!(class_available -> classes (class_id));
joinable!(class_available -> days (day));
joinable!(class_subjects -> classes (class));
joinable!(class_subjects -> subjects (subject));
joinable!(class_timetable -> activities (activities));
joinable!(class_timetable -> classes (class_id));
joinable!(class_timetable -> days (day_id));
joinable!(classes -> school (school));
joinable!(post -> school (school));
joinable!(post -> users (sender));
joinable!(school -> city (city));
joinable!(school -> school_type (school_type));
joinable!(school -> town (town));
joinable!(school -> users (manager));
joinable!(school_grade -> school_type (school_type));
joinable!(school_menu -> school_type (school_type));
joinable!(school_subjects -> school (school));
joinable!(school_subjects -> subjects (subject));
joinable!(school_time -> days (day));
joinable!(school_time -> hours (hour));
joinable!(school_time -> school (school));
joinable!(school_users -> school (school_id));
joinable!(school_users -> users (user_id));
joinable!(subjects -> school_type (school_type));
joinable!(teacher_available -> days (day));
joinable!(town -> city (city));

allow_tables_to_appear_in_same_query!(
    activities,
    auth,
    city,
    class_available,
    class_subjects,
    class_timetable,
    classes,
    content_type,
    days,
    hours,
    post,
    school,
    school_grade,
    school_menu,
    school_subjects,
    school_time,
    school_type,
    school_users,
    session,
    subjects,
    teacher_available,
    teachers_menu,
    town,
    users,
);
