# Display available commands
default:
  @just -f {{justfile()}} --list

complete slug:
  #!/usr/bin/env sh
  curl -s \
    -w '{"status": %{http_code}}' \
    -X PUT \
    http://localhost:3000/todos/{{ slug }} \
  | jq -s 'add'

create title:
  #!/usr/bin/env sh
  curl -s \
    -X POST \
    -w '{"status": %{http_code}}' \
    http://localhost:3000/todos \
    -H 'Content-Type: application/json' \
    -d '{"title": "{{ title }}"}' \
  | jq -s 'add'

delete slug:
  #!/usr/bin/env sh
  curl -s \
    -w '{"status": %{http_code}}' \
    -X DELETE \
    http://localhost:3000/todos/{{ slug }} \
  | jq -s 'add'

get slug:
  #!/usr/bin/env sh
  curl -s \
    -w '{"status": %{http_code}}' \
    -X GET http://localhost:3000/todos/{{ slug }} \
  | jq -s 'add'

flush:
  #!/usr/bin/env sh
  if [ -f todos.db ]; then
    rm -f todos.db
  else
    echo "❎ Database not found."
  fi 

list:
  #!/usr/bin/env sh
  curl -s -X GET http://localhost:3000/todos | jq .

start:
  #!/usr/bin/env sh
  if [ -f cargo.pid ]; then
    echo "❎ Already running (PID: $(cat cargo.pid))"
    exit 1
  fi
  cargo run &
  echo $! > cargo.pid
  echo "✅ Started with PID: $(cat cargo.pid)"

stop:
  #!/usr/bin/env sh
  if [ ! -f cargo.pid ]; then
    echo "❎ No cargo.pid file found"
    exit 1
  fi
  PID=$(cat cargo.pid)
  if kill "$PID" 2>/dev/null; then
    echo "✅ Stopped PID: $PID"
  else
    echo "❎ Process $PID not found (stale pid file)"
  fi
  rm -f cargo.pid
