var net = require('net');


var client = new net.Socket();
client.connect(7878, '127.0.0.1', function() {

    let i = BigInt("422151" , 10);
    let j = BigInt("429486429654" , 10);
    
    let  f = Buffer.concat(
        [conv64(i) , conv64(j)]);
	client.write(f);
});

client.on('data', function(data) {
	console.log('Received: ' + data);
	client.destroy(); // kill client after server's response
});

client.on('close', function() {
	console.log('Connection closed');
});


const conv = num => Buffer.from([
    (num >> 24) & 255,
    (num >> 16) & 255,
    (num >> 8) & 255,
    num & 255,
  ]);

  const conv64 = num => {
    const buf = Buffer.allocUnsafe(8);

    buf.writeBigInt64BE(num, 0);

    return buf;
  }