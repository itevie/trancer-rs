use serenity::all::User;

pub fn pronoun<S: Into<String>>(user1: &User, user2: &User, same_prn: S, diff_prn: S) -> String {
    let same_prn = same_prn.into();
    let diff_prn = diff_prn.into();

    if user1 == user2 {
        if same_prn == "$name" {
            user1.name.clone()
        } else {
            same_prn
        }
    } else {
        if diff_prn == "$name" {
            user2.name.clone()
        } else {
            diff_prn
        }
    }
}