import React from 'react';

// Needed because of some javascript weirdness
import 'babel-polyfill';

const js = import("../../crypto_module");

class Chat extends React.Component {
    constructor(props) {
        super(props);
        this.state = {
          messages: [],
          message: '',
          encrypt: '',
        };

        this.send = this.send.bind(this);
        this.encrypt = this.encrypt.bind(this);
        this.onChangeMessage = this.onChangeMessage.bind(this);
        this.onChangeEncrypt = this.onChangeEncrypt.bind(this);
      }

    async componentDidMount() {
        // Call the rust code from js
        const crypto = await js;

        const seedOne = this.generateSeed();
        const seedTwo = this.generateSeed();
        const keypair = crypto.Keypair.new(seedOne, seedTwo);

        const socket = require('socket.io-client')('https://enigmatic-savannah-85282.herokuapp.com/');

        // Stupid hack for accessing this in the socket events
        const obj = this;

        // Actions for when connecting to server
        socket.on('connect', function(){
            // For registering with the channel on connect
            socket.emit('REGISTER', keypair.public_key_display_wasm());

            const temp = obj.state.messages;

            temp.push({
                message: "Connected successfully to server.",
                bgColor: 'white',
                color: 'green',
            });

            obj.setState({
                messages: temp,
            });
        });

        // For displaying all chat room messages
        socket.on('MESSAGE', function(data){
            const temp = obj.state.messages;

            if (data.split(":\n")[0] == `[${keypair.public_key_display_wasm()}]`) {
                const plaintext = data.split(":\n")[1].slice(1).trim();
                console.log(plaintext)
                try {
                    const decrypted = obj.state.keypair.decrypt(plaintext);
                    alert("You've got mail!");
                    temp.push({
                        message: `${decrypted}`,
                        bgColor: 'green',
                        color: 'white',
                    });
                } catch(err) {
                    temp.push({
                        message: "Error decrypting...",
                        bgColor: 'red',
                        color: 'black',
                    });
                }
            } else {
                temp.push({
                    message: data,
                    bgColor: 'white',
                    color: 'gray',
                });
            }
            obj.setState({
                messages: temp,
            });
        });

         // For displaying welcome message
         socket.on('WELCOME', function(data){
            const temp = obj.state.messages;

            temp.push({
                message: data,
                bgColor: 'gray',
                color: 'black',
            });
            
            obj.setState({
                messages: temp,
            });
        });


        // For displaying new registration when new users connect
        socket.on('NEW_REGISTRATION', function(data){
            const temp = obj.state.messages;

            temp.push({
                message: `User joined: ${data}`,
                bgColor: 'yellow',
                color: 'black',
            });
            
            obj.setState({
                messages: temp,
            });
        });

        // For displaying when new users disconnect
        socket.on('DISCONNECTED', function(data){
            const temp = obj.state.messages;
            
            temp.push({
                message: `User left: ${data}`,
                bgColor: 'red',
                color: 'white',
            });

            obj.setState({
                messages: temp,
            });
        });

        // Actions for when disconnecting from server
        socket.on('disconnect', function(){
            socket.emit('DISCONNECTED', this.state.keypair.public_key_display_wasm())
            const temp = obj.state.messages;
            
            temp.push({
                message: `You disconnected from server.`,
                bgColor: 'pink',
            });

            obj.setState({
                messages: temp,
            });
        });

        this.setState({
            socket,
            keypair,
            crypto,
        })
    }

    componentDidUpdate() {
        this.scrollToBottom();
    }

    // Thanks to StackOverFlow (@metakermit) for this one!
    // https://stackoverflow.com/questions/37620694/how-to-scroll-to-bottom-in-react
    scrollToBottom = () => {
        this.messagesEnd.scrollIntoView({ behavior: "smooth" });
    }

    generateSeed() {
        let array = new Uint32Array(32);
        window.crypto.getRandomValues(array);

        return array;
    }

    onChangeMessage(e) {
        if (e.target.value == "" ||
            e.target.value.match(/^\d+$/) && 
            Number(e.target.value) < 2147483647 &&
            Number(e.target.value) > -1
        ) {
            this.setState({
                message: e.target.value,
            })
        }
    }

    send() {
        this.state.socket.emit('MESSAGE', this.state.message);
        this.setState({
            message: '',
        });
    }

    onChangeEncrypt(e) {
        this.setState({
            encrypt: e.target.value,
        })
    }

    encrypt() {
        try {
            const n = this.state.encrypt.trim();
            const encrypted = this.state.crypto.encrypt(this.state.message, n);
            
            this.setState({
                encrypt: '',
                message: `[${this.state.encrypt}]:\n${encrypted}`,
            });
        } catch(err) {
            this.setState({
                encrypt: 'Failure to encrypt',
            });
        }
    }

    render() {
        return (
            <div>
                <ul id="messages">
                    {this.state.messages.map(x => <li style={{backgroundColor: x.bgColor, color: x.color}}>{x.message}</li>)}
                </ul>
                <form action="">
                    <div className="inputbox">
                        <input autoComplete="off" onChange={this.onChangeMessage} value={this.state.message}/>
                        <button onClick={this.send} type="button">Send</button>
                    </div>
                    <div className="inputbox">
                        <input autoComplete="off" onChange={this.onChangeEncrypt} value={this.state.encrypt}/>
                        <button onClick={this.encrypt} type="button">Encrypt</button>
                    </div>
                </form>
                <div style={{ float:"left", clear: "both" }}
                    ref={(el) => { this.messagesEnd = el; }}>
                </div>
            </div>
        );
    }
}

export default Chat;
