var net = require('net');


var client = new net.Socket();
client.connect(7878, '127.0.0.1', function() {

    let x =  Math.ceil( Math.random() * 10 );
    let tab = [];
    tab.push(Buffer.from([x]));
    console.log("writing n characters" + x);
    for (let i = 0; i <  x ; i++ ) {
        let num1 = Math.ceil( Math.random() * 1000000 );
        let num2 = Math.ceil( Math.random() * 1000000) ;
        let i = BigInt( num1 , 10);
        let j = BigInt(num2, 10);
        tab.push(conv64(i));
        tab.push(conv64(j));
        console.log(num1 + ',' + num2);
    }
    
    
    let  f = Buffer.concat(tab);
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