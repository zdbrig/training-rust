var net = require('net');


var client = new net.Socket();
client.connect(7878, '127.0.0.1', function() {

    let x =  Math.ceil( Math.random() * 10 );
    let tab = [];
    tab.push(Buffer.from([x]));
    console.log("writing n characters" + x);
 

 // tab 32 64 strin(100) bool   
    for (let i = 0; i <  x ; i++ ) {
        let num1 = Math.ceil( Math.random() * 1000000 );
        let num2 = Math.ceil( Math.random() * 1000000) ;
        let str = ("hello word karima ").charCodeAt(0);

       let bufferstr =  Buffer.from("hello word karima     " , "ascii");
       console.log("str to asci "+str);
        let i =  num1 ;
        let j = BigInt(num2, 10);
        tab.push(conv32(i));
        tab.push(conv64(j));
        tab.push(bufferstr);
        tab.push(Buffer.from([1]));
        console.log(num1 + ',' + num2);
    }
    let  f1 = Buffer.concat(tab);
	client.write(f1);
});


// client.on('data', function(data) {
// 	console.log('Received: ' + data);

// 	client.destroy(); // kill client after server's response
// });

// client.on('close', function() {
// 	console.log('Connection closed');
// });
client.on('data', function(data) {
  let b = Buffer.from(data);
console.log('Received: ' + b[0]);

  let  n = b[0];

  for (let i = 0; i < (n * 22 + 1 ); i = i + 22) {
console.log("tabb"+b[i])
      let num1 = b.readUInt32BE(i+1);
      let num2 = b.readBigUInt64BE(i+5);
      let bol =b[i+13]==1;
      let str=b.slice([i+14..i+21]);
      console.log("str"+str)
      console.log(num1 + ',' + num2,bol,str);
  }
client.destroy(); // kill client after server's response
});

client.on('close', function() {
console.log('Connection closed');
});





  const conv64 = num => {
    const buf = Buffer.allocUnsafe(8);

    buf.writeBigInt64BE(num, 0);

    return buf;
  }
  
const conv32 = num => {
    const buf = Buffer.allocUnsafe(4);

    buf.writeUInt32BE(num, 0);

    return buf;
  }