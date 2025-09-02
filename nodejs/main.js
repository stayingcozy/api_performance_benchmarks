const express = require('express');
const { Pool } = require('pg');
const Redis = require('ioredis');
const axios = require('axios');
const app = express();
app.use(express.json());
const pool = new Pool({ connectionString: process.env.POSTGRES_DSN });
const redis = new Redis(process.env.REDIS_URL);
app.post('/order', async (req, res) => {
  const { user_id, product_id, amount, currency } = req.body;
  try {
    // Redis GET
    await redis.get(`user:${user_id}`);
    // External API call
    let rate = 1.0;
    try {
      const resp = await axios.get(`http://external-currency-api/rate?to=${currency}`);
      rate = resp.data.rate || 1.0;
    } catch (e) {
      console.warn("External API failed:", e.message);
    }
    // DB INSERT
    await pool.query(
      'INSERT INTO orders (user_id, product_id, amount, currency) VALUES ($1, $2, $3, $4)',
      [user_id, product_id, amount * rate, currency]
    );
    res.send('ok');
  } catch (err) {
    console.error(err);
    res.status(500).send('error');
  }
});
app.listen(3000, () => {
  console.log('Server on :3000');
});