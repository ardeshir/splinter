% SPLINTER-CIRCUIT-PROPOSE(1) Cargill, Incorporated | Splinter Commands

NAME
====

**splinter-circuit-propose** — Propose that a new circuit is created

SYNOPSIS
========
**splinter circuit propose** \[**FLAGS**\] \[**OPTIONS**\]

DESCRIPTION
===========
This command submits a proposal to create a new Splinter circuit with one or more
other nodes. When the other nodes receive this proposal, they vote to accept or
reject it with `splinter circuit vote`. If all nodes accept the proposal, the
circuit is created.

It is necessary to specify the participating nodes for a circuit proposal as well
as information for the intended use and operation of the circuit. Circuit proposals
may be constructed multiple ways, via command-line options using this command or
using the `splinter circuit template` command. The `splinter circuit template`
command offers a partially completed circuit proposal, requiring less input than
using `splinter circuit propose`. More information on how to use circuit templates
can be found in the splinter-circuit-template(1) man page.

FLAGS
=====
`-n`, `--dry-run`
: Show the circuit definition without submitting the proposal

`-h`, `--help`
: Prints help information

`-q`, `--quiet`
: Decrease verbosity (the opposite of -v). When specified, only errors or
  warnings will be output.

`-V`, `--version`
: Prints version information

`-v`
: Increases verbosity (the opposite of -q). Specify multiple times for more
  output.

OPTIONS
=======
`--comments <comments>`
: Adds human-readable comments to the circuit proposal.

`-k, --key <private-key-file>`
: Specifies the full path to the private key file.

`--management <management_type>`
: Specifies the circuit management type. Circuit management type indicates the
  application authorization handler which handles the circuit’s change proposals.

`--metadata <application_metadata>` ...
: Provides application-specific metadata for the circuit proposal. Repeat this
  option to provide multiple entries for the application metadata.

`--metadata-encoding <metadata_encoding>`
: Sets the encoding type for the application metadata (default: `string`).
  Accepted values: `json`, `string`.

`--node <node_string>` ...
: Specifies a node that should be part of the circuit, using the format
  `<node_id>::<endpoint>`. This node ID must match the node ID and endpoint entry
  in the node registry. The proposer must also specify its own node, if it is to
  be included on the circuit proposal. Repeat this option to specify multiple
  nodes.

`--service <service_string>` ...
: Specifies the service ID and allowed nodes, using the format
  `<service_id>::<allowed_nodes>`. Service IDs are comprised of 4 ASCII alphanumeric
  characters. The <allowed_nodes> specifies the node which the service will run
  on, currently only one node ID is allowed.

`--service-arg <service_argument>` ...
: Passes key/value arguments to the specified service (as defined by
  `--service`), using the format `<service_id>::<key>=<value>`. Service arguments
  provided must match those required to create the service. The glob operator,
  `*`, may be used in place of the <service_id> to match all or certain parts
  of the 4 character <service_id>. For instance, `AA*::<key>=<value>` to match
  all service IDs that begin with `AA`. Repeat this option to specify multiple
  key/value arguments.

`--service-peer-group <service_peer_group>` ...
: Specifies the service peer group (a list of peer services). Peer services are
  services used by peer nodes within a circuit. This is the group of services
  that must come to consensus amongst the node peers. Repeat this option to
  specify multiple service peer groups.

`--service-type <service_type>` ...
: Provides a service type for the specified service (as defined by
  `--service`), using the format `<service_id>::<service_type>`. The glob operator,
  `*`, may be used in place of the <service_id> to match all or certain parts
  of the 4 character <service_id>. For instance, `AA*::<service_type>` to match
  all service IDs that begin with `AA`. Scabbard is a  Splinter service currently
  implemented to be used, that can be specified with `service_type` of `scabbard`.
  Repeat this option to specify multiple service types.

`--template <template>`
: Specifies a template to use for defining the circuit. Additional information
  on circuit templates can be found in the splinter-circuit-template(1) man page.

`--template-arg <template_arg>` ...
: Provides a key/value argument for the circuit template (as specified by
  `--template``), using the format `<key>=<value>`. Repeat this option to
  specify multiple template arguments.

`-U`, `--url <url>`
: Specifies the URL for the `splinterd` REST API. The URL is required unless
  `$SPLINTER_REST_API_URL_ENV` is set.

ENVIRONMENT VARIABLES
=====================
**SPLINTER_REST_API_URL_ENV**
: URL for the `splinterd` REST API. (See `-U`, `--url`.)

EXAMPLES
========
This command proposes a simple circuit with one other node.

* The proposing node has ID `alpha001` and endpoint `tls://splinterd-node-acme001:8044`.
* The other node has ID `beta001` and endpoint `tls://splinterd-node-beta001:8044`.
* There is one service with ID `AA01`. This service has no service
  arguments, service type, or service group.

```
$ splinter circuit propose \
  --node alpha001::tls://splinterd-node-alpha001:8044 \
  --node beta001::tls://splinterd-node-beta001:8044 \
  --service AA01::alpha001 \
  --key <private-key-file>
  --url <URL-of-your-splinterd-REST-API>
```

The next command proposes a circuit with one other node, with multiple services
and multiple service-args.

* The proposing node has ID `alpha001` and endpoint `tls://splinterd-node-acme001:8044`.
* The other node has ID `beta001` and endpoint `tls://splinterd-node-beta001:8044`.
* There are two services for each member node with a `service-type` of `scabbard`
  for each. The service ID for the alpha node service is `AA01` and the
  beta node service ID is `BB01`. Each of these services are specified
  in the service group by providing each service ID for the `service-peer-group`
  argument. There is also a service-arg for the `AA01`, the `admin_keys`
  which is required by the Splinter Scabbard service.

```
splinter circuit propose \
  --key <private-key-file> \
  --url <URL-of-your-splinterd-REST-API> \
  --node alpha001::tls://splinterd-node-alpha001:8044 \
  --node beta001::tls://splinterd-node-beta001:8044 \
  --service AA01::alpha-node-001 \
  --service BB01::beta-node-001 \
  --service-type AA01::scabbard \
  --service-type BB01::scabbard \
  --service-arg AA01::admin_keys=<node-public-key> \
  --service-arg BB01::admin_keys=<node-public-key> \
  --service-peer-group AA01,BB01
```

The glob operator, `*` may be used to match <service_id> for the `--service-type`
and `--service-arg` arguments. Therefore, this part of the command:

```
--service-type AA01::scabbard \
--service-type BB01::scabbard \
--service-arg AA01::admin_keys=<node-public-key> \
--service-arg BB01::admin_keys=<node-public-key> \
```

becomes the following using the glob operator:
```
--service-type *::scabbard \
--service-arg *::admin_keys=<node-public-key> \
```

SEE ALSO
========
| `splinter-circuit-proposals(1)`
| `splinter-circuit-template(1)`
| `splinter-circuit-vote(1)`
|
| Splinter documentation: https://github.com/Cargill/splinter-docs/blob/master/docs/index.md