use engine_share::entity::restful::SysResult;

pub async fn test() -> SysResult<&'static str, String> {
    SysResult::ok("hello client")
}