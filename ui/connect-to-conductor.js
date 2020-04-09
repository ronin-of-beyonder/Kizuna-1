// gratitude to https://flaviocopes.com/react-electron/ for this approach
const net = require("net");
const childProcess = require("child_process");
const port = 8888;
const client = new net.Socket();

let startedConductor = false;
const tryConnection = () => {
  client.connect({ port }, () => {
    client.end();
    if (!startedConductor) {
      console.log("starting ui, connect to:" + port);
      startedConductor = true;
      const exec = childProcess.exec;
      exec("ionic serve");
    }
  });
};

tryConnection();

client.on("error", () => {
  console.log("Waiting for Holochain to configure and boot");
  setTimeout(tryConnection, 5000);
});
