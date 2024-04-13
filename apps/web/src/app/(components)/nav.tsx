import { useSession } from "next-auth/react";

export function Nav() {
    return (
        <nav className="flex justify-between">
            <div>
                <User />
            </div>
        </nav>
    );
}

function User() {
    const session = useSession();
    return (
        <div>
            {session.data && (
                <img
                    src={session.data.user.image!}
                    alt="user"
                    className="rounded-full border border-white/40"
                />
            )}
        </div>
    );
}
