The purpose of this tool is to fingerprint and test for various behaviours of
NATs so that we can gather information on what kinds of behaviours exist in the
wild.

The following is an incomplete list of information that should be tested for:

# Mapping protocols supported

We should test for the following protocols, as well as gather specific
information about how they are implemented (eg. exact format of responses).

## IGD

Some things to test for:

(a) What is the exact format of responses (TODO: elaborate on this question
    after becoming more familiar with the protocol)

(b) Does the IGD allow mapping internal ports to different external ports

(c) Does the IGD allow non-permanent port leases?

(d) What methods does the IGD support?

## NAT-PMP

TODO

## PCP

TODO

# Port mapping behaviour

Here are some behaviours of NATs that we want to test for.

(a) Does the NAT give the same external endpoint to packets sent from the same
    internal endpoint to different IP addresses?

(b) Does the NAT give the same external endpoint to packets sent from the same
    internal endpoint to different ports at the same remote IP address?

(c) Does the NAT allocate new IP addresses sequentially? Randomly? Some combination?

(d) For NATs that do not satisy (a) or (b), is their allocation behaviour the
    same when allocating ports for internal endpoints they already have
    mappings for vs new internal endpoints that they need to create a fresh
    mapping for?

(e) Will the NAT allow incoming traffic from endpoints it has not sent data to?
    What about if the IP has been contacted, just not that port?

(f) If a NAT does not satisy (e), does it continue to block traffic from
    that endpoint even after it has sent traffic to it and created a mapping for
    it? How long for?

(h) Does the mapped external port match the internal port where possible?

(i) Does the NAT support hair-pinning?

