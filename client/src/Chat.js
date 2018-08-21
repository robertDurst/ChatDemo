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
          decrypt: '',
        };

        this.send = this.send.bind(this);
        this.encrypt = this.encrypt.bind(this);
        this.decrypt = this.decrypt.bind(this);
        this.onChangeMessage = this.onChangeMessage.bind(this);
        this.onChangeEncrypt = this.onChangeEncrypt.bind(this);
        this.onChangeDecrypt = this.onChangeDecrypt.bind(this);
      }

    async componentDidMount() {
        // Call the rust code from js
        const crypto = await js;

        const seedOne = this.generateSeed();
        const seedTwo = this.generateSeed();
        const keypair = crypto.Keypair.new(seedOne, seedTwo);

        const socket = require('socket.io-client')('http://localhost:3001');

        // Stupid hack for accessing this in the socket events
        const obj = this;

        // Actions for when connecting to server
        socket.on('connect', function(){
            // For registering with the channel on connect
            socket.emit('REGISTER', keypair.public_key_display_wasm());

            const temp = obj.state.messages;
            temp.push("Connected successfully to server.");
            obj.setState({
                messages: temp,
            });
        });

        // For displaying all chat room messages
        socket.on('MESSAGE', function(data){
            const temp = obj.state.messages;
            temp.push(data);
            obj.setState({
                messages: temp,
            });
        });

        // For displaying new registration when new users connect
        socket.on('NEW_REGISTRATION', function(data){
            const temp = obj.state.messages;
            temp.push(`User joined: ${data}`);
            obj.setState({
                messages: temp,
            });
        });

        // For displaying when new users disconnect
        socket.on('DISCONNECTED', function(data){
            const temp = obj.state.messages;
            temp.push(`User left: ${data}`);
            obj.setState({
                messages: temp,
            });
        });

        // Actions for when disconnecting from server
        socket.on('disconnect', function(){
            socket.emit('DISCONNECTED', 'name')
            const temp = obj.state.messages;
            temp.push("Disconnected from server.");
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

    generateSeed() {
        let seed = [];
        for (var i = 0; i < 32; i ++) {
            seed.push(Math.floor(Math.random() * 100));
        }

        return seed;
    }

    onChangeMessage(e) {
        this.setState({
            message: e.target.value,
        })
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
            const keypair = this.state.encrypt.replace(/\(|\)/g, "").split(",");
            const e = keypair[0].trim();
            const n = keypair[1].trim();
            const encrypted = this.state.crypto.encrypt(this.state.message, e, n);
            
            this.setState({
                encrypt: '',
                message: encrypted,
            });
        } catch(err) {
            this.setState({
                encrypt: 'Failure to encrypt',
            });
        }
    }

    onChangeDecrypt(e) {
        this.setState({
            decrypt: e.target.value,
        })
    }

    decrypt() {
        try {
            const decrypted = this.state.keypair.decrypt(this.state.decrypt);
            this.setState({
                decrypt: decrypted,
            });
        } catch(err) {
            this.setState({
                decrypt: 'Unsuccessful decryption.',
            });
        }
    }

    render() {
        return (
            <div>
                <ul id="messages">
                    {this.state.messages.map(x => <li>{x}</li>)}
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
                    <div className="inputbox">
                        <input autoComplete="off" onChange={this.onChangeDecrypt} value={this.state.decrypt}/>
                        <button onClick={this.decrypt} type="button">Decrypt</button>
                    </div>
                </form>
            </div>
        );
    }
}

export default Chat;
