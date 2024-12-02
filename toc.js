// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded affix "><li class="part-title">Hermes v1.10.4</li><li class="chapter-item expanded affix "><li class="spacer"></li><li class="chapter-item expanded "><a href="index.html"><strong aria-hidden="true">1.</strong> Introduction</a></li><li class="chapter-item expanded "><a href="quick-start/index.html"><strong aria-hidden="true">2.</strong> Quick start</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="quick-start/pre-requisites.html"><strong aria-hidden="true">2.1.</strong> Prerequisites</a></li><li class="chapter-item expanded "><a href="quick-start/installation.html"><strong aria-hidden="true">2.2.</strong> Installation</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/index.html"><strong aria-hidden="true">3.</strong> Tutorials</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/pre-requisites/index.html"><strong aria-hidden="true">3.1.</strong> Prerequisites for local chains</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/pre-requisites/gaia.html"><strong aria-hidden="true">3.1.1.</strong> Install Gaia</a></li><li class="chapter-item expanded "><a href="tutorials/pre-requisites/gaiad-manager.html"><strong aria-hidden="true">3.1.2.</strong> Install Gaiad Manager</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/local-chains/index.html"><strong aria-hidden="true">3.2.</strong> Local chains</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/local-chains/start-local-chains.html"><strong aria-hidden="true">3.2.1.</strong> Start the local chains</a></li><li class="chapter-item expanded "><a href="tutorials/local-chains/add-a-new-relay-path.html"><strong aria-hidden="true">3.2.2.</strong> Add a new relay path</a></li><li class="chapter-item expanded "><a href="tutorials/local-chains/start-relaying.html"><strong aria-hidden="true">3.2.3.</strong> Start relaying</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/more-chains/index.html"><strong aria-hidden="true">3.3.</strong> More Local Chains</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/more-chains/start-local-chains.html"><strong aria-hidden="true">3.3.1.</strong> Start the local chains</a></li><li class="chapter-item expanded "><a href="tutorials/more-chains/build-the-topology.html"><strong aria-hidden="true">3.3.2.</strong> Build the topology</a></li><li class="chapter-item expanded "><a href="tutorials/more-chains/start-relaying.html"><strong aria-hidden="true">3.3.3.</strong> Start relaying</a></li><li class="chapter-item expanded "><a href="tutorials/more-chains/concurrent-instances.html"><strong aria-hidden="true">3.3.4.</strong> Add new instances of Hermes</a></li></ol></li><li class="chapter-item expanded "><a href="tutorials/production/index.html"><strong aria-hidden="true">3.4.</strong> Relaying in production</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="tutorials/production/setup-grafana.html"><strong aria-hidden="true">3.4.1.</strong> Set up the monitoring platform</a></li><li class="chapter-item expanded "><a href="tutorials/production/setup-hermes.html"><strong aria-hidden="true">3.4.2.</strong> Set up Hermes</a></li><li class="chapter-item expanded "><a href="tutorials/production/start-relaying.html"><strong aria-hidden="true">3.4.3.</strong> Start relaying</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="documentation/configuration/index.html"><strong aria-hidden="true">4.</strong> Configuration</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/configuration/comet-compat-mode.html"><strong aria-hidden="true">4.1.</strong> CometBFT Compatibility modes</a></li><li class="chapter-item expanded "><a href="documentation/configuration/configure-hermes.html"><strong aria-hidden="true">4.2.</strong> Configure Hermes</a></li><li class="chapter-item expanded "><a href="documentation/configuration/description.html"><strong aria-hidden="true">4.3.</strong> Description of the parameters</a></li><li class="chapter-item expanded "><a href="documentation/configuration/dynamic-gas-fees.html"><strong aria-hidden="true">4.4.</strong> Dynamic gas fees</a></li><li class="chapter-item expanded "><a href="documentation/configuration/filter-incentivized.html"><strong aria-hidden="true">4.5.</strong> Filter incentivized packets</a></li><li class="chapter-item expanded "><a href="documentation/configuration/packet-clearing.html"><strong aria-hidden="true">4.6.</strong> Packet clearing</a></li><li class="chapter-item expanded "><a href="documentation/configuration/performance.html"><strong aria-hidden="true">4.7.</strong> Performance tuning</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/telemetry/index.html"><strong aria-hidden="true">5.</strong> Telemetry</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/telemetry/operators.html"><strong aria-hidden="true">5.1.</strong> Operators guide</a></li><li class="chapter-item expanded "><a href="documentation/telemetry/integration.html"><strong aria-hidden="true">5.2.</strong> Integration</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/rest-api.html"><strong aria-hidden="true">6.</strong> REST API</a></li><li class="chapter-item expanded "><a href="advanced/index.html"><strong aria-hidden="true">7.</strong> Advanced</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="advanced/features.html"><strong aria-hidden="true">7.1.</strong> Features</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/index.html"><strong aria-hidden="true">7.2.</strong> Troubleshooting</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="advanced/troubleshooting/help-command.html"><strong aria-hidden="true">7.2.1.</strong> Help Command</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/profiling.html"><strong aria-hidden="true">7.2.2.</strong> Profiling</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/log-level.html"><strong aria-hidden="true">7.2.3.</strong> Log level</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/patch-gaia.html"><strong aria-hidden="true">7.2.4.</strong> Patch Gaia</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/inspect.html"><strong aria-hidden="true">7.2.5.</strong> Inspecting the relayer&#39;s state</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/cross-comp-config.html"><strong aria-hidden="true">7.2.6.</strong> Cross Stack Misconfiguration</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/genesis-restart.html"><strong aria-hidden="true">7.2.7.</strong> Genesis restart without IBC upgrade proposal</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/clock-drift.html"><strong aria-hidden="true">7.2.8.</strong> Handling Clock Drift</a></li><li class="chapter-item expanded "><a href="advanced/troubleshooting/gas-errors.html"><strong aria-hidden="true">7.2.9.</strong> Gas Errors</a></li></ol></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/index.html"><strong aria-hidden="true">8.</strong> Commands Reference</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/global.html"><strong aria-hidden="true">8.1.</strong> Global options and JSON output</a></li><li class="chapter-item expanded "><a href="documentation/commands/keys/index.html"><strong aria-hidden="true">8.2.</strong> Keys</a></li><li class="chapter-item expanded "><a href="documentation/commands/config.html"><strong aria-hidden="true">8.3.</strong> Generating and Validating Config Files</a></li><li class="chapter-item expanded "><a href="documentation/commands/path-setup/index.html"><strong aria-hidden="true">8.4.</strong> Path setup</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/path-setup/clients.html"><strong aria-hidden="true">8.4.1.</strong> Clients</a></li><li class="chapter-item expanded "><a href="documentation/commands/path-setup/connections.html"><strong aria-hidden="true">8.4.2.</strong> Connections</a></li><li class="chapter-item expanded "><a href="documentation/commands/path-setup/channels.html"><strong aria-hidden="true">8.4.3.</strong> Channels</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/relaying/index.html"><strong aria-hidden="true">8.5.</strong> Relaying</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/relaying/packets.html"><strong aria-hidden="true">8.5.1.</strong> Packet Messages</a></li><li class="chapter-item expanded "><a href="documentation/commands/relaying/handshakes.html"><strong aria-hidden="true">8.5.2.</strong> Handshake Messages</a></li><li class="chapter-item expanded "><a href="documentation/commands/relaying/clear.html"><strong aria-hidden="true">8.5.3.</strong> Clearing Packets</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/listen/index.html"><strong aria-hidden="true">8.6.</strong> Listen mode</a></li><li class="chapter-item expanded "><a href="documentation/commands/upgrade/index.html"><strong aria-hidden="true">8.7.</strong> Client upgrade</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/upgrade/test.html"><strong aria-hidden="true">8.7.1.</strong> Testing client upgrade</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/forwarding/index.html"><strong aria-hidden="true">8.8.</strong> Packet Forwarding</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/forwarding/test.html"><strong aria-hidden="true">8.8.1.</strong> Testing packet forwarding</a></li><li class="chapter-item expanded "><a href="documentation/forwarding/legacy_test.html"><strong aria-hidden="true">8.8.2.</strong> Testing legacy packet forwarding</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/misbehaviour/index.html"><strong aria-hidden="true">8.9.</strong> Misbehaviour</a></li><li class="chapter-item expanded "><a href="documentation/commands/queries/index.html"><strong aria-hidden="true">8.10.</strong> Queries</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/queries/client.html"><strong aria-hidden="true">8.10.1.</strong> Client</a></li><li class="chapter-item expanded "><a href="documentation/commands/queries/connection.html"><strong aria-hidden="true">8.10.2.</strong> Connection</a></li><li class="chapter-item expanded "><a href="documentation/commands/queries/channel.html"><strong aria-hidden="true">8.10.3.</strong> Channel</a></li><li class="chapter-item expanded "><a href="documentation/commands/queries/packet.html"><strong aria-hidden="true">8.10.4.</strong> Packet</a></li><li class="chapter-item expanded "><a href="documentation/commands/queries/tx.html"><strong aria-hidden="true">8.10.5.</strong> Tx</a></li><li class="chapter-item expanded "><a href="documentation/commands/queries/transfer.html"><strong aria-hidden="true">8.10.6.</strong> Transfer</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/tx/index.html"><strong aria-hidden="true">8.11.</strong> Transactions</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/tx/connection.html"><strong aria-hidden="true">8.11.1.</strong> Connection</a></li><li class="chapter-item expanded "><a href="documentation/commands/tx/channel-open.html"><strong aria-hidden="true">8.11.2.</strong> Channel Open</a></li><li class="chapter-item expanded "><a href="documentation/commands/tx/channel-close.html"><strong aria-hidden="true">8.11.3.</strong> Channel Close</a></li><li class="chapter-item expanded "><a href="documentation/commands/tx/packet.html"><strong aria-hidden="true">8.11.4.</strong> Packet</a></li><li class="chapter-item expanded "><a href="documentation/commands/tx/upgrade.html"><strong aria-hidden="true">8.11.5.</strong> Upgrade</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/fee/index.html"><strong aria-hidden="true">8.12.</strong> ICS29 Fee</a></li><li><ol class="section"><li class="chapter-item expanded "><a href="documentation/commands/fee/register-counterparty-payee.html"><strong aria-hidden="true">8.12.1.</strong> Register Counterparty Payee</a></li><li class="chapter-item expanded "><a href="documentation/commands/fee/register-payee.html"><strong aria-hidden="true">8.12.2.</strong> Register Payee</a></li><li class="chapter-item expanded "><a href="documentation/commands/fee/transfer.html"><strong aria-hidden="true">8.12.3.</strong> Transfer</a></li></ol></li><li class="chapter-item expanded "><a href="documentation/commands/logs/index.html"><strong aria-hidden="true">8.13.</strong> Logs</a></li></ol></li><li class="chapter-item expanded "><a href="glossary.html"><strong aria-hidden="true">9.</strong> Glossary</a></li><li class="chapter-item expanded affix "><li class="spacer"></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString();
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
