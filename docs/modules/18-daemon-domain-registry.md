# 18 Daemon Domain Registry

## Purpose

Daemon Domain Registry owns daemon-scoped domain identity, domain-qualified agent addressing, and cross-daemon route resolution.

zterm can run multiple daemons. AgentTeam must support communication across those daemon domains without letting Agent Registry, Communication Center, zterm/tmux Adapter, or CLI invent competing domain formats.

## Owns

- Daemon domain id validation.
- Local daemon domain identity.
- Remote daemon registry.
- Domain-qualified agent address parsing.
- Domain alias resolution.
- Cross-daemon route resolution.
- Domain endpoint snapshot for Debug Center.
- Domain help text.

## Does Not Own

- Agent local name allocation.
- Agent lifecycle status projection.
- Message delivery queue state.
- Task state.
- zterm/tmux transport execution.
- Config TOML parsing.
- Error classification.
- Persistence append implementation.

## Module Function Map

| function_id | Owner | Purpose | Input | Output | Required red tests |
|---|---|---|---|---|---|
| `domain.validate_id` | Daemon Domain Registry | Validate daemon domain id and alias | domain config | `DomainReq02Validated` | invalid/duplicate domain |
| `domain.register_local` | Daemon Domain Registry + Config Center | Register local daemon domain identity | config domain section | local domain fact | missing local domain |
| `domain.register_remote` | Daemon Domain Registry + Config Center | Register remote daemon endpoint metadata | config domain section | remote domain fact | duplicate remote domain |
| `domain.parse_agent_addr` | Daemon Domain Registry | Parse domain-qualified agent address | raw target | `DomainAgentAddr03Resolved` | ambiguous bare cross-domain target |
| `domain.resolve_route` | Daemon Domain Registry + Communication Center | Resolve target domain before message/task route | route intent | domain route plan | comm owns domain rules |
| `domain.project_address` | Daemon Domain Registry + Agent Registry | Build stable agent address projection | domain id + agent name | `agent@domain` projection | local name used globally |
| `domain.snapshot` | Daemon Domain Registry | Provide domain registry snapshot to Debug Center | registry state | domain snapshot | endpoint secret leak |
| `domain.help` | Daemon Domain Registry | Describe domain addressing and cross-daemon rules | help topic | help model | hidden endpoint dependency |

## Module Help Contract

Required help topics:

```text
agentteam help domain
agentteam help domain local
agentteam help domain remote
agentteam help domain address
agentteam help domain route
agentteam help domain red-tests
```

Help content must explain:

- every daemon has a stable `domain_id`
- local agent names are unique only inside one domain
- cross-daemon agent references use domain-qualified addresses
- canonical address format is `agent@domain`
- bare agent names are valid only when target domain is already local and unambiguous
- Communication Center asks Domain Registry before cross-daemon delivery
- zterm/tmux Adapter consumes resolved daemon endpoint facts; it does not resolve business targets

Help content must not:

- let Communication Center invent domain parsing rules
- let Agent Registry treat local names as globally unique
- expose auth tokens in examples or snapshots
- suggest fallback to local daemon when remote domain lookup fails

## Public API Boundary

```text
DomainReq01RawTarget -> DomainReq02Validated -> DomainAgentAddr03Resolved -> DomainRoute04Plan
```

Only Daemon Domain Registry validates domain ids, parses domain-qualified agent addresses, and resolves domain route plans.

Only Agent Registry allocates local names inside a resolved domain.

Only Communication Center routes business envelopes after the target domain is resolved.

Only zterm/tmux Adapter talks to the resolved daemon endpoint.

## Addressing Rules

Canonical agent address:

```text
agent@domain
```

Examples:

```text
Kevin@local
Alice@agentteam-main
Bob@review-daemon
agentteam_worker_21@agentteam-main
```

Rules:

- `domain` is a daemon domain id, not a project slug.
- Local aliases may include `local`, but `local` must resolve to exactly one configured daemon domain.
- Bare names such as `Alice` are allowed only for local-domain commands where no remote domain context is implied.
- Cross-daemon commands must use `agent@domain`, `role:<role>@domain`, `team:<team_id>@domain`, or `all@domain`.
- Domain id and alias uniqueness is enforced by Daemon Domain Registry.
- Agent Registry must not allocate globally unique names; it allocates names inside one domain/team scope.

## Cross-Daemon Route Flow

```text
CLI / agent message
  |
  v
Input Gateway
  |
  v
Communication Center
  |
  v
Daemon Domain Registry
  |  parse target + resolve domain
  v
DomainRoute04Plan
  |
  +--> local domain: Communication Center local delivery
  |
  +--> remote domain: zterm/tmux Adapter or daemon client uses resolved endpoint
```

No module may silently reroute a failed remote-domain target to local delivery.

## Config Shape

Config Center parses domain config and passes normalized domain facts to Daemon Domain Registry.

```text
[daemon_domain]
id
aliases

[[daemon_domains.remote]]
id
aliases
host
port
auth_token
```

Auth tokens are user config and must be redacted in snapshots/output.

## Error Behavior

Invalid, duplicate, missing, or unreachable domain facts emit DomainRegistry fault facts to Error Center.

Remote domain lookup failure is explicit failure. It must not fallback to local domain delivery.

## Debug Snapshot

Snapshot includes:

- local domain id
- domain aliases
- remote domain ids
- endpoint host/port
- token redaction status
- latest route resolution results
- failed domain lookups

## Resource Lifecycle

Daemon Domain Registry owns lifecycle requests for:

- domain registry snapshot
- remote daemon route handle
- remote daemon health observation handle

Rules:

- Register route handle before cross-daemon delivery starts.
- Release route handle after local/remote delivery accepts or classified failure persists.
- Remote health observations must be bounded and released after debug/route operation completes.
- A remote route handle without Communication Center envelope is an orphan candidate.
- Route count, failed lookup count, remote health latency, and endpoint count are efficiency metrics.

## Red Tests

- Duplicate domain id fails.
- Missing local domain id fails.
- Bare remote agent target fails as ambiguous.
- Communication Center parsing domain address directly fails architecture gate.
- Agent Registry treating local names as globally unique fails architecture gate.
- Remote domain lookup fallback to local fails.
- Domain snapshot leaking auth token fails.
- Cross-daemon route without route handle lease fails.
- zterm/tmux Adapter resolving business target domain fails architecture gate.

## Open Decisions

- Whether cross-daemon delivery uses AgentTeam daemon HTTP directly, zterm bridge, or both as separate transport adapters.
- Whether domain trust policy is allowlist-only in v1.
