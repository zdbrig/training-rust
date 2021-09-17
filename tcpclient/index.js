var net = require('net');

var client = new net.Socket();
client.connect(7878, '127.0.0.1', function() {
    let  f = Buffer.from([43]);
	client.write(f);
});

client.on('data', function(data) {
	console.log('Received: ' + data);
	client.destroy(); // kill client after server's response
});

client.on('close', function() {
	console.log('Connection closed');
});