import { sealSession } from "$lib/auth/cookieSession.js";

const hardcodedPassword = "ivansearchInlogger54";

export async function POST({ request }) {
  try {
    const { password } = await request.json();
    if (password === hardcodedPassword) {
      const cookie = await sealSession({ email: "admin" });
      return new Response(null, {
        status: 200,
        headers: {
          "Set-Cookie": cookie,
        },
      });
    }
    return new Response("Invalid credentials", { status: 401 });
  } catch (error) {
    console.error("Error while logging in: ", error);
    return new Response("Internal server error", { status: 500 });
  }
}
