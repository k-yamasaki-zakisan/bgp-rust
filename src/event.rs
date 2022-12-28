/// BGPのRFC内 8.1
/// (https://datatracker.ietf.org/doc/html/rfc4271#section8.1)で
/// 定義されているEvent)を表す列挙型

#[derive(PartialEq, Eq, Debug, Clone, Hash)]
pub enum Event {
    ManualStart,
    // 正常系歯科実装しない本実装では別のEventとして扱う意味がないため
    TcpConnectionConfirmed,
}
