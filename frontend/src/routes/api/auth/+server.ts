import { sealSession } from "$lib/auth/cookieSession.js";

const hardcodedPassword = "test";

export async function GET({ request }) {
  try {
    const password = request.headers.get("password");
    if (password === hardcodedPassword) {
      const cookie = await sealSession({ user: "admin" });
      return new Response(null, {
        status: 200,
        headers: {
          Cookie: cookie,
        },
      });
    } else {
      return new Response("Invalid credentials", { status: 401 });
    }
  } catch (error) {
    console.error("Error while logging in: ", error);
    return new Response("Internal server error", { status: 500 });
  }
}
