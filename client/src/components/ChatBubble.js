import React from 'react';
import PropTypes from 'prop-types';

class ChatBubble extends React.Component {
    render() {
        const {
            bgColor,
            color,
            children
        } = this.props;

        return (
            <div className='chat-bubble' style={{ backgroundColor: bgColor,  color: color}}>
                {children}
            </div>
        );
    }
}

ChatBubble.propTypes = {
    children: PropTypes.node
}

export default ChatBubble;
