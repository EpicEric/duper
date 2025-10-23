import duper


def test_basic():
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
                        roles: ["admin", "user"],
                        metadata: Metadata({
                            last_login: DateTime("2024-01-15T10:30:00Z"),
                            ip: IPV4("173.255.230.79"),
                        }),
                    }),
                ],
            },
        })
    """

    obj = duper.loads(DUPER_DATA)
    serialized = duper.dumps(obj)
    assert (
        serialized
        == r"""APIResponse({status: 200, headers: {content_type: "application/duper", cache_control: "max-age=3600"}, body: {users: [User({id: Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"), name: "Alice", email: Email("alice@example.com"), roles: ["admin", "user"], metadata: Metadata({last_login: DateTime("2024-01-15T10:30:00Z"), ip: IPV4("173.255.230.79")})})]}})"""
    )
    assert serialized == obj.model_dump(mode="duper")
