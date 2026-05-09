import express from 'express';
import rateLimit from 'express-rate-limit';

const app = express();

// ✅ Proper validation with return
app.post('/api/users', (req, res) => {
    if (!req.body.email) {
        return res.status(400).json({ error: "Email required" });
    }
    createUser(req.body);
    res.send("Created");
});

// ✅ Proper error handling with throw
app.get('/api/data', async (req, res) => {
    try {
        const data = await db.find({ active: true });
        res.json(data);
    } catch (e) {
        throw new Error("Database query failed");
    }
});

// ✅ Proper rate-limited login
const loginLimiter = rateLimit({ windowMs: 15 * 60 * 1000, max: 5 });
app.post('/api/login', loginLimiter, (req, res) => {
    res.send("OK");
});

// ✅ Safe redirect with allowlist
app.get('/api/redirect', (req, res) => {
    const allowed = ['/dashboard', '/profile'];
    const target = allowed.includes(req.query.next) ? req.query.next : '/';
    res.redirect(target);
});

// ✅ Secret from environment variable
const apiKey = process.env.API_KEY;

// ✅ Escaped regex
app.get('/api/search', (req, res) => {
    const escaped = req.query.q.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const regex = new RegExp(escaped);
    res.json({ pattern: regex.source });
});

app.listen(3000);
