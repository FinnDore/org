import { cn } from "@/lib/utils";
import { Session } from "next-auth";
import { forwardRef, type HTMLProps } from "react";

export const Pfp = forwardRef<
    HTMLDivElement,
    HTMLProps<HTMLDivElement> & { user: Session["user"] }
>(function Pfp({ user, ...props }, ref) {
    return (
        <div
            ref={ref}
            {...props}
            className={cn("aspect-square", props.className)}
        >
            <div className="relative h-full w-full cursor-pointer transition-all hover:scale-110">
                <div className="absolute z-10 h-full w-full overflow-clip rounded-full border border-black/20 dark:border-white/40">
                    <picture className="block h-[70px] min-h-full w-[70px]  min-w-full overflow-clip">
                        <source srcSet="/NOISE.webp" type="image/webp" />
                        <img className="aspect-square min-h-full" aria-hidden />
                    </picture>
                </div>
                <picture className="absolute block h-full w-full overflow-clip rounded-full blur-md saturate-150">
                    <source srcSet={user.image ?? "TODO fallback image"} />
                    <img
                        className="block h-full w-full rounded-full"
                        alt={`profile picture for ${user.name ?? "a user"}`}
                    />
                </picture>
                <picture className="absolute block h-full w-full overflow-clip rounded-full saturate-150">
                    <source srcSet={user.image ?? "TODO fallback image"} />
                    <img className="h-full w-full" aria-hidden />
                </picture>
            </div>
        </div>
    );
});
