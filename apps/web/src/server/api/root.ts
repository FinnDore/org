import { sceneRouter } from "@/server/api/routers/scene";
import { createCallerFactory, createTRPCRouter } from "@/server/api/trpc";
import { meRouter } from "./routers/me";
import { orgRouter } from "./routers/org";

/**
 * This is the primary router for your server.
 *
 * All routers added in /api/routers should be manually added here.
 */
export const appRouter = createTRPCRouter({
    scene: sceneRouter,
    me: meRouter,
    org: orgRouter,
});

// export type definition of API
export type AppRouter = typeof appRouter;

/**
 * Create a server-side caller for the tRPC API.
 * @example
 * const trpc = createCaller(createContext);
 * const res = await trpc.post.all();
 *       ^? Post[]
 */
export const createCaller = createCallerFactory(appRouter);
