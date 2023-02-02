<script lang="ts">
    import { python, bash } from "svelte-highlight/languages";

    import Blog from "$lib/components/Blog.svelte";
    import Codeblock from "$lib/components/Codeblock.svelte";

    import Kine from "$lib/assets/kine.png";
	import ChromeImage from '$lib/assets/chrome_P5Xo6HKINX.png';
</script>

<Blog title="Why I Made Kine">
    <img src={ Kine } width="50%" alt="Logo of Kine"/>
    <p>
        <a class="link" href="https://github.com/zomatree/kine">Link to the Github.</a>
    </p>
    <p>
        Python has been used for making web apps for a long time, Instagram for example is a giant Django program and people continue to make websites using frameworks such as Flask and Django,
        however most of these options rely on using static HTML templates, which are cumbersome to work with, they only allow passing in static values and hard to update, you must either refresh
        the page which is jarring for the user or require using javascript which makes using python for the website not worth it, at this point it would be easier to start making your web app in
        a javascript framework instead.
    </p>
    <p>
        This is why i set out to create Kine.
    </p>
    <h2>
        What is Kine?
    </h2>
    <p>
        The name Kine is short for kinetic, as chemical reactions can be achieved with sufficient kinetic energy, and reactivity is a large part of Kine. Kine is a framework i created to bring what
        creating interactive web apps should be like,
        no more HTML templates, instead it takes what libraries such as ReactJS do with components and diffing nodes. This means the entire website can be updated without needing to refresh or use
        javascript to fetch new infomation and manually update the content of elements.
    </p>
    <h2>
        A simple example program
    </h2>
    <p>
        Making a program in Kine is very easy, it can be done in under 20 lines!

        <Codeblock language={ python } code={`from kine import *
from kine.renderers.web import *

import asyncio

@component
def app(cx: Scope):
    return cx.render(p["Hello World!"])

asyncio.run(start_web(app()))`}/>

        This code doesnt do much, all it does is display "Hello World!" in the browser but its a simple example and shows off the basic structure of a program.
    </p>

    <p>
        The example is using the <span class="inline-codeblock">web</span> backend which means the python code is ran on the server and uses a websocket to send the updates to the client, this is useful if you
        need to interact with a database or other sensative data.
    </p>

    <h2>What makes Kine standout</h2>

    <p>
        Recently there have been a couple other libraries which do similar things to Kine, <a class="link" href="https://github.com/pynecone-io/pynecone">Pynecone</a> and
        <a class="link" href="https://github.com/zauberzeug/nicegui/">NiceGUI</a>
        have both seen large traction, however these libraries tend to shy away from giving you control and vary in performance.
    </p>
    <p>
        A feature which makes Kine standout is the ability to compile the entire project into a WebAssembly app, meaning that you can convert an entire python web page into a static client side page, this is great
        for when you dont need to connect directly to a database or dont want to be bogged down by being required to run a server.
    </p>

    <h2>How to make a Kine WebAssembly app</h2>
    <p>
        The steps to make a WebAssembly app is very simple, Kine comes with a CLI which can be used to create the project and build the project into the static content

        <Codeblock language={ bash } code={`\
python -m pip install kine      # installs the library
python -m kine new my_kine_app  # creates the project
cd my_kine_app                  # navigates to the project folder
python -m kine build            # build the project
python -m kine serve            # runs a web server to serve the built web app`}/>

        After this you can head to your browser similar to before and the entire python app is ran on the browser.

    </p>
    <img class="rounded-image" src = { ChromeImage } width="60%"/>
    <p></p>
</Blog>
