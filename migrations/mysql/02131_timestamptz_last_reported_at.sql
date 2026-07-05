-- migrate:up
-- MySQL datetime type does not have timezone support.
-- The Go code fix (serverPushStatusLogic.go, serverPushUserTrafficLogic.go)
-- removing .UTC() is sufficient for MySQL environments.
SELECT 1;

-- migrate:down
SELECT 1;

