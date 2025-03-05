/*
curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=$GOOGLE_API_KEY" \
    -H 'Content-Type: application/json' \
    -X POST \
    -d '{
      "contents": [{
        "parts":[{"text": "Write a story about a magic backpack."}]
        }]
       }'

curl "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:streamGenerateContent?alt=sse&key=${GOOGLE_API_KEY}" \
        -H 'Content-Type: application/json' \
        --no-buffer \
        -d '{ "contents":[{"parts":[{"text": "Write a story about a magic backpack."}]}]}'

*/

pub const DEFAULT_CACHE_DIR: &str = ".cache/";

pub const ASCII_ART: &str = "
(\\
\\'\\
 \\'\\     __________
 / '|   ()_________)
 \\ '/    \\ ~~~~~~~~ \\
   \\       \\ ~~~~~~   \\
   ==).      \\__________\\
  (__)       ()__________)";

pub const TEXT_TO_TEXT_API_URL: &str =
    "https://generativelanguage.googleapis.com/v1beta/models/gemini-1.5-flash:generateContent?key=";
