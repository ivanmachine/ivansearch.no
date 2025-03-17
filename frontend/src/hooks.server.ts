import { redirect, type Handle } from "@sveltejs/kit";
import { unsealSession } from "./lib/auth/cookieSession";

const unrestrictedPaths = ["/login", "/"];

// TODO: reset cookie on error, it ain't working now
export const handle: Handle = async ({ event, resolve }) => {
  if (unrestrictedPaths.includes(event.url.pathname))
    return await resolve(event);
  // Check authentication for restricted paths
  try {
    console.log("Inside try");
    const auth_session = event.cookies.get("auth_session");
    const userSession = await unsealSession(auth_session);
    if (userSession) return await resolve(event);
  } catch (e) {
    console.log("Inside catch");
    console.error("Error while getting session: ", e);
    event.cookies.delete("auth_session", { path: "/" });
  }

  // If not authenticated or error occurred, redirect to login
  console.log("Middleware server: redirecting user /login");
  return redirect(302, "/login");
};
