const express = require('express');
const rendertron = require('rendertron-middleware');

const app = express();

app.use(rendertron.makeMiddleware({
	proxyUrl: 'http://127.0.0.1:3000',
}));

app.use(express.static('../client/dist'));

app.get('/hello', (req, res) => res.send('Hello World!'))

app.listen(8090);

- To change `User-Agent` https://addons.mozilla.org/en-US/firefox/addon/modheader-firefox/
