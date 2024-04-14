import { neon } from "@neondatabase/serverless";

async function getRedirectURL(alias) {
  const db_url = process.env.DATABASE_URL;
  if (!db_url) {
    throw new Error("Missing env variable DATABASE_URL");
  }
  const sql = neon(db_url);
  const response = await sql`SELECT url FROM aliases WHERE alias=${alias}`;
  return response;
}

export async function GET(request) {
  const url = new URL(request.url);
  const alias = url.pathname.split("/").splice(1);
  if (alias.length > 1) {
    return new Response(`Not Found`, {
      status: 404,
    });
  }

  const response = await getRedirectURL(alias[0]);

  if (response.length === 0 || !response[0].url) {
    return new Response(`Not Found`, {
      status: 404,
    });
  }

  return Response.redirect(response[0].url, 307);
}
