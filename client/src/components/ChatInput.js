import React from 'react';

class ChatInput extends React.Component {
    render() {
        return (
            <div className="inputbox">
                <input autoComplete="off" onChange={this.props.onChange} value={this.props.value}/>
                <button onClick={this.props.onClick} type="button">{ this.props.children }</button>
            </div>
        )
    }
}

export default ChatInput;
