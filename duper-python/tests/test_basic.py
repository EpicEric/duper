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

    python_dict = duper.loads(DUPER_DATA)
    serialized = duper.dumps(python_dict)
    # TO-DO: Remove DuperUnknown
    assert (
        serialized
        == r"""APIResponse({status: 200, headers: DuperUnknown({content_type: "application/duper", cache_control: "max-age=3600"}), body: DuperUnknown({users: [User({id: Uuid("7039311b-02d2-4849-a6de-900d4dbe9acb"), name: "Alice", email: Email("alice@example.com"), roles: ["admin", "user"], metadata: Metadata({last_login: DateTime("2024-01-15T10:30:00Z"), ip: IPV4("173.255.230.79")})})]})})"""
    )
