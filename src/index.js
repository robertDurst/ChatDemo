import React from 'react';
import ReactDOM from 'react-dom';
import Chat from './Chat';

const title = 'My Minimal React Webpack Babel Setup';

ReactDOM.render(<Chat/>, document.getElementById('app'));

module.hot.accept();
