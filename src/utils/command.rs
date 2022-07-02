pub enum UserCommand {
    RECURSIVE(u64, u64),
    DYNAMIC(u64),
    //recursive then dynamic
    BOTH(((u64, u64), u64)),
}
