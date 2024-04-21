use teloxide::types::User;
use teloxide::utils::markdown;

pub(crate) fn markup_username_with_link(user: &User) -> String {
    markdown::link(
        &format!("tg://user?id={}", user.id),
        &markdown::escape(&user.full_name()),
    )
}

pub(crate) fn plain_link(url: &str) -> String {
    markdown::link(url, &markdown::escape(url))
}
