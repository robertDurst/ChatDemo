import React from 'react';

class ChatInput extends React.Component {
    render() {
        return (
            <div className="inputbox">
                <input 
                    autoComplete="off" 
                    onChange={this.props.onChange} 
                    value={this.props.value}
                    disabled = {true}
                    placeholder={this.props.placeholder}
                    disabled={this.props.inputDisabled}
                />
                <button 
                    onClick={this.props.onClick} 
                    type="button"
                    style={{
                        display: !this.props.buttonDisabled ? "none" : "",
                        cursor: "pointer",
                    }}
                >
                    {this.props.children}
                </button>
            </div>
        )
    }
}

export default ChatInput;
