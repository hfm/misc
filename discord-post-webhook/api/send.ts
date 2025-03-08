import {Hono} from 'hono'
import {handle} from 'hono/vercel'

const app = new Hono().basePath('/api')

const DISCORD_WEBHOOK_URL = process.env.DISCORD_WEBHOOK_URL || ''

app.post('/post', async (c) => {
  try {
    const response = await fetch(DISCORD_WEBHOOK_URL, {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ content: 'Hello, World!',}),
    })

    if (!response.ok) {
      return c.text('Failed to send message');
    }
    return c.text('Message sent!');

  } catch (error) {
    console.error(error)
  }
})

export const POST = handle(app)
