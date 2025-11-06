import pytest
import duper


DUPER_DATA = """
APIResponse({
    status: 200,
    headers: {
        content_type: "application/duper",
        cache_control: "max-age=3600",
    },
    body: {
        users: [
            User({
                id: Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"),
                name: "Alice",
                email: Email("alice@example.com"),
                roles: [
                    "admin",
                    "user",
                ],
                metadata: Metadata({
                    last_login: Instant('2024-01-15T10:30:00Z'),
                    ip: IPV4("173.255.230.79"),
                }),
            }),
        ],
    },
})"""


def test_basic():
    obj = duper.loads(DUPER_DATA)
    serialized = duper.dumps(obj)
    assert (
        serialized
        == r"""APIResponse({status: 200, headers: {content_type: "application/duper", cache_control: "max-age=3600"}, body: {users: [User({id: Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"), name: "Alice", email: Email("alice@example.com"), roles: ["admin", "user"], metadata: Metadata({last_login: Instant('2024-01-15T10:30:00Z'), ip: IPV4("173.255.230.79")})})]}})"""
    )
    assert serialized == obj.model_dump(mode="duper")

    assert duper.dumps(obj, indent=4) == DUPER_DATA.strip()

    assert (
        duper.dumps(obj, strip_identifiers=True)
        == r"""{status: 200, headers: {content_type: "application/duper", cache_control: "max-age=3600"}, body: {users: [{id: "7039311b-02d2-4849-a6de-900d4dbe9acb", name: "Alice", email: "alice@example.com", roles: ["admin", "user"], metadata: {last_login: '2024-01-15T10:30:00Z', ip: "173.255.230.79"}}]}}"""
    )

    assert (
        duper.dumps(obj, minify=True)
        == r"""APIResponse({status:200,headers:{content_type:"application/duper",cache_control:"max-age=3600"},body:{users:[User({id:Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"),name:"Alice",email:Email("alice@example.com"),roles:["admin","user"],metadata:Metadata({last_login:Instant('2024-01-15T10:30:00Z'),ip:IPV4("173.255.230.79")})})]}})"""
    )

    assert (
        duper.dumps(obj, strip_identifiers=True, minify=True)
        == r"""{status:200,headers:{content_type:"application/duper",cache_control:"max-age=3600"},body:{users:[{id:"7039311b-02d2-4849-a6de-900d4dbe9acb",name:"Alice",email:"alice@example.com",roles:["admin","user"],metadata:{last_login:'2024-01-15T10:30:00Z',ip:"173.255.230.79"}}]}}"""
    )

    with pytest.raises(
        ValueError, match="cannot stringify with both indent and minify options"
    ):
        _ = duper.dumps(obj, indent=2, minify=True)
