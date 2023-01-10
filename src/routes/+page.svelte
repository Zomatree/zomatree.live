<script lang="ts">
    import cake from "$lib/assets/cake.jpg";
    import og_image from "$lib/assets/og_image.png";

    export function scroll_to_blog() {
        let bottom = document.getElementById("bottom")!;
        bottom.scrollIntoView({behavior: "smooth"});
    }
</script>

<svelte:head>
    <meta property="og:description" content="👋 I'm a programmer who works with Python, Rust and Javascript.">
    <meta property="og:image" content={ og_image }>
    <link rel="canonical" href= { og_image }>
    <link rel="image_src" href= { og_image }>
    <meta name="twitter:card" content="summary_large_image">
</svelte:head>

<div class="main">
    <div class="intro">
        <div class="intro-inner">
            <div class="name">
                Hi.
                <div class="emphasis">
                    I'm Zomatree.
                </div>
                <div class="intro-small-text">
                    A programmer who works with Python, Rust and Javascript.
                </div>
            </div>
            <a class="github-link" href="https://github.com/zomatree" target="_blank" rel="nooopener noreferrer">github.com/Zomatree »</a>
        </div>
        <div class="down-arrow" on:click={scroll_to_blog}>
            <svg fill="#CAB8FD" height="20px" width="20px" version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 330 330" xml:space="preserve"><g id="XMLID_169_"><path id="XMLID_197_" d="M304.394,139.394l-139.39,139.393L25.607,139.393c-5.857-5.857-15.355-5.858-21.213,0.001c-5.858,5.858-5.858,15.355,0,21.213l150.004,150c2.813,2.813,6.628,4.393,10.606,4.393s7.794-1.581,10.606-4.394l149.996-150c5.858-5.858,5.858-15.355,0-21.213C319.749,133.536,310.251,133.535,304.394,139.394z"/><path id="XMLID_221_" d="M154.398,190.607c2.813,2.813,6.628,4.393,10.606,4.393s7.794-1.581,10.606-4.394l149.996-150c5.858-5.858,5.858-15.355,0-21.213c-5.857-5.858-15.355-5.858-21.213,0l-139.39,139.393L25.607,19.393c-5.857-5.858-15.355-5.858-21.213,0c-5.858,5.858-5.858,15.355,0,21.213L154.398,190.607z"/></g></svg>
        </div>
    </div>
    <div class="blog-section">
        <h1 class="header" id="blogs">
            Blog Posts
        </h1>
        <div class="blogs">
            <a class="blog-card" href="/blog/cake">
                <img class="blog-card-image" alt="Slice of chocolate cake on white plate" src={cake}/>
                <h3 class="blog-card-title">Cake</h3>
            </a>
            <a class="blog-card" href="/blog/cake">
                <img class="blog-card-image" alt="Slice of chocolate cake on white plate" src={cake}/>
                <h3 class="blog-card-title">Cake</h3>
            </a>
            <a class="blog-card" href="/blog/cake">
                <img class="blog-card-image" alt="Slice of chocolate cake on white plate" src={cake}/>
                <h3 class="blog-card-title">Cake</h3>
            </a>
            <a class="blog-card" href="/blog/cake">
                <img class="blog-card-image" alt="Slice of chocolate cake on white plate" src={cake}/>
                <h3 class="blog-card-title">Cake</h3>
            </a>
            <a class="blog-card" href="/blog/cake">
                <img class="blog-card-image" alt="Slice of chocolate cake on white plate" src={cake}/>
                <h3 class="blog-card-title">Cake</h3>
            </a>
        </div>
    </div>
    <div id="bottom"/>
</div>
