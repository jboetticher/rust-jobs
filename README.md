# Rust Job Board Keywords
This is just a simple project that takes all of the job postings on the rust job board site [rustjobs.dev](https://rustjobs.dev) and figures out which keywords show up the most in the descriptions. Some notable ones are:  

- "software": 92
- "systems": 80
- "code": 67
- "blockchain": 66
- "protocol": 55
- "crypto": 28
- "hardware": 27
- "cloudflare": 26

I was originally going to use a web scraper, but then I found out that the Firebase query that the website uses isn't origin protected (as of Dec 19th 2022). This is probably fine for the site since it wouldn't mind its job postings being used everywhere.  

This was a simple task that I took as my first serious mini-project in Rust, so the code here might be a little questionable to more seasoned Rust developers.