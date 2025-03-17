import { sealData, unsealData } from "iron-session";

const SESSION_PASSWORD = "sajhbgvbjihgvhcfY&%RTYU(/Y&TRDFGYUHIYGFTGHVBJUHYFT)"; // TODO: fix later
// Can fail !!!!
export async function unsealSession(
  auth_session: string | undefined
): Promise<UserSession | null> {
  if (!auth_session) return null;
  // Will be empty object {} if no cookie is set ***NOT STRING***
  const unsealedCookie: UserSession = await unsealData(auth_session, {
    password: SESSION_PASSWORD,
  });
  return unsealedCookie?.user ? unsealedCookie : null;
}

// Can fail !!!!
export async function sealSession(user: UserSession) {
  const encryptedSession = sealData(user, {
    password: SESSION_PASSWORD,
  });
  return encryptedSession;
}

export type UserSession = {
  user: string;
};
