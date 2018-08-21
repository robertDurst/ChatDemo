const app = require('express')();
const http = require('http').Server(app);
const io = require('socket.io')(http);

const port = process.env.PORT || 3001;

io.on('connection', function(socket){
  // For announcing the connection of new users by public key
  socket.on('REGISTER', function(data){
    console.log("User joined:", data);
    socket.emit('MESSAGE', `Welcome!\nYour public key: ${data}`);
    socket.broadcast.emit("NEW_REGISTRATION", data);
  });

  // For displaying and broadcasting all chat messages
  socket.on('MESSAGE', function(data){
    console.log(data);
    io.emit('MESSAGE', data);
  });

   // For displaying and broadcasting all when users disconnect
   socket.on('DISCONNECTED', function(data){
    console.log("User left:", data);
    io.emit('DISCONNECTED', data);
  });
});

http.listen(port, function(){
  console.log('listening on *:' + port);
});