const express = require('express');
const app = express();
const port = 3000;

app.get('/api/resource', (req, res) => {
  res.json({ message: 'This is mock data!' });
});

app.listen(port, () => {
  console.log(`Mock server running at http://localhost:${port}`);
});