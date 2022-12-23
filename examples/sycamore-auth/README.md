# Sycamore With Supabase Auth Example

[Sycamore](https://sycamore-rs.netlify.app/) is an reactive library for creating web apps in Rust and WebAssembly. [Supabase](https://sycamore-supabase-js-rs-auth-demo.netlify.app/) is an open source Firebase alternative.

Live demo [sycamore-supabase-js-rs-auth-demo.netlify.app](https://sycamore-supabase-auth.netlify.app)

## Running

Add enironment variables

```
SUPABASE_URL={ACCESS_TOKEN}
SUPABASE_KEY={SUPABASE_KEY}
```

of if you on PowerShell

```
$Env:SUPABASE_URL="{ACCESS_TOKEN}"
$Env:SUPABASE_KEY="{SUPABASE_KEY}"
```

and then

```
trunk serve
```
