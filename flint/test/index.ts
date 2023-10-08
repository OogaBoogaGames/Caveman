import Bao from "baojs";
import serveStatic from "serve-static-bun";

const app = new Bao();

app.get(
    "/flint/*any",
    serveStatic("../pkg", { middlewareMode: "bao", stripFromPathname: "/flint" }),
);

app.get("/", (ctx) => {
    return ctx.sendRaw(new Response(Bun.file("./public/index.html")))
});

let server = app.listen({port: 3000});
console.log(`Listening on ${server.hostname}:${server.port}`);