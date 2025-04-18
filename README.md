# API Aggregator for Static Sites

A Rust-based API aggregator that provides endpoints for Spotify recently played tracks, Letterboxd watched movies, and URL webhook functionality. This service can be used to add dynamic content to static websites.

## Setup

1. Clone the repository
2. Create a `.env` file with the following variables:
   ```
   API_KEY=your_api_key_here
   SPOTIFY_CLIENT_ID=your_spotify_client_id
   SPOTIFY_CLIENT_SECRET=your_spotify_client_secret
   SPOTIFY_REFRESH_TOKEN=your_spotify_refresh_token
   ```
3. Generate an API key with the provided script:
   ```
   python generate_api_key.py
   ```
4. Build and run the application:
   ```
   cargo build --release
   cargo run --release
   ```

## API Endpoints

All endpoints require authentication with the API key in the Authorization header:
```
Authorization: Bearer your_api_key_here
```

### URL Webhook Endpoint

#### POST /url-webhook
Records a URL provided in the request body.

**Request:**
- Method: POST
- Body: Raw text containing the URL

**Response:**
- 200 OK: Successfully recorded the URL
- 401 Unauthorized: Invalid or missing API key

#### GET /url-webhook
Returns the 5 most recently recorded URLs.

**Request:**
- Method: GET

**Response:**
- 200 OK: JSON containing the URLs array
- 401 Unauthorized: Invalid or missing API key

Response Format:
```json
{
  "urls": ["url1", "url2", "url3", "url4", "url5"]
}
```

### Letterboxd Endpoint

#### GET /letterboxd
Returns the 5 most recently watched movies from a Letterboxd RSS feed.

**Request:**
- Method: GET
- Query Parameters:
  - `feed_url` (optional): URL of the Letterboxd RSS feed (default: https://letterboxd.com/atropos_Dad/rss)
  - `no_cache` (optional): Set to "true" to bypass cache

**Response:**
- 200 OK: JSON containing the movies array
- 401 Unauthorized: Invalid or missing API key
- 500 Internal Server Error: Unable to fetch or parse the feed

Response Format:
```json
{
  "movies": [
    {
      "title": "Movie Title with Rating",
      "link": "https://letterboxd.com/user/film/movie-slug/",
      "description": "Review text",
      "pub_date": "Wed, 01 Jan 2023 12:00:00 +0000",
      "film_title": "Movie Title",
      "rating": "3.5",
      "rewatch": "true"
    },
    ...
  ]
}
```

### Spotify Endpoint

#### GET /spotify
Returns the most recently played tracks from Spotify.

**Request:**
- Method: GET
- Query Parameters:
  - `limit` (optional): Number of tracks to return (default: 5)
  - `no_cache` (optional): Set to "true" to bypass cache

**Response:**
- 200 OK: JSON containing the tracks array
- 401 Unauthorized: Invalid or missing API key
- 500 Internal Server Error: Unable to fetch tracks from Spotify

Response Format:
```json
{
  "tracks": [
    {
      "track_name": "Track Name",
      "artist": "Artist Name",
      "album_name": "Album Name",
      "played_at": "2023-01-01T12:00:00Z",
      "spotify_url": "https://open.spotify.com/track/id",
      "album_image_url": "https://i.scdn.co/image/id"
    },
    ...
  ]
}
```

## Caching

Both the Letterboxd and Spotify endpoints implement caching to improve performance and reduce external API calls:

- Letterboxd data is cached for 1 hour
- Spotify data is cached for 15 minutes

Use the `no_cache=true` query parameter to bypass the cache when needed.

## Error Handling

All endpoints return appropriate HTTP status codes and error messages in JSON format when issues occur. 