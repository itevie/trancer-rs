use serenity::all::User;

pub fn pronoun<S: Into<String>>(user1: &User, user2: &User, same_prn: S, diff_prn: S) -> String {
    let same_prn = same_prn.into();
    let diff_prn = diff_prn.into();

    if user1 == user2 {
        same_prn.replace("$name", &user1.name)
    } else {
        diff_prn.replace("$name", &user2.name)
    }
}
