console.log("Connecting...");
const ws = new WebSocket("ws://127.0.0.1:9090");
let interval = 0;
ws.onopen = () => {
  console.log("Connected");
  const buf = new Uint8Array(514);
  buf[0] = 0x01;
  let c = 0;
  interval = setInterval(() => {
    buf[2 + c] = 0;
    c = (c + 1) % 512;
    buf[2 + c] = 255;
    ws.send(buf);
  }, 10);
};
ws.onclose = () => {
  console.log("Disconnected");
  clearInterval(interval);
};
