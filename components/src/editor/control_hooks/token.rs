use std::fmt::Display;

pub enum MarkdownToken {
    Bold,
    Italic,
    Underline,
    Link,
    AlignLeft,
    AlignCenter,
    AlignRight,
    ListBullet,
    NumberedList,
    H1,
    H2,
    H3,
    Code,
}

impl Display for MarkdownToken {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MarkdownToken::Bold => write!(f, "Bold"),
            MarkdownToken::Italic => write!(f, "Italic"),
            MarkdownToken::Underline => write!(f, "Underline"),
            MarkdownToken::Link => write!(f, "Link"),
            MarkdownToken::AlignLeft => write!(f, "AlignLeft"),
            MarkdownToken::AlignCenter => write!(f, "AlignCenter"),
            MarkdownToken::AlignRight => write!(f, "AlignRight"),
            MarkdownToken::ListBullet => write!(f, "ListBullet"),
            MarkdownToken::NumberedList => write!(f, "NumberedList"),
            MarkdownToken::H1 => write!(f, "H1"),
            MarkdownToken::H2 => write!(f, "H2"),
            MarkdownToken::H3 => write!(f, "H3"),
            MarkdownToken::Code => write!(f, "Code"),
        }
    }
}
