<script lang="ts">
    import ProjectCard from "$lib/components/ProjectCard.svelte";
    import BlogCard from "$lib/components/BlogCard.svelte";

    import cake from "$lib/assets/cake.jpg";
    import og_image from "$lib/assets/og_image.png";

    export function scroll_to_blog() {
        let bottom = document.getElementById("bottom")!;
        bottom.scrollIntoView({behavior: "smooth"});
    }
</script>

<svelte:head>
    <meta property="og:description" name="og:description" content="👋 I'm a programmer who works with Python, Rust and Typescript.">
    <meta property="description" name="description" content="👋 I'm a programmer who works with Python, Rust and Typescript.">
    <meta property="og:image" name="og:image" content={ og_image }>
    <meta property="og:title" name="og:title" content="Zomatree">
    <link rel="image_src" href= { og_image }>
    <meta property="twitter:card" name="twitter:card" content="summary_large_image">
    <meta property="theme-color" name="theme-color" content="#e0d5ff">
    <meta property="og:site_name" name="og:site_name" content="Zomatree">
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
                    A programmer who works with Python, Rust and Typescript.
                </div>
                <div class="icon-row">
                    <svg xmlns="http://www.w3.org/2000/svg" fill="var(--primary-3)" width=32 height=32 viewBox="0 0 32 32"><path d="M19.212 26.447l-0.287 0.412-0.1 0.512 0.1 0.512 0.287 0.412 0.412 0.287 0.512 0.1 0.512-0.1 0.412-0.287 0.287-0.412 0.1-0.512-0.1-0.512-0.287-0.412-0.412-0.287-0.512-0.1-0.512 0.1zM27.297 8.64l0.35 0.075 0.4 0.15 0.437 0.225 0.45 0.337 0.45 0.437 0.437 0.587 0.4 0.737 0.35 0.912 0.262 1.1 0.175 1.3 0.062 1.537-0.075 1.537-0.2 1.3-0.3 1.075-0.4 0.887-0.45 0.712-0.5 0.562-0.525 0.412-0.525 0.3-0.5 0.2-0.45 0.112-0.4 0.062-0.3 0.025-0.2-0.012h-10.272v1.025h7.298l0.012 3.449 0.025 0.45-0.062 0.425-0.137 0.387-0.212 0.362-0.312 0.312-0.387 0.3-0.475 0.25-0.55 0.212-0.637 0.187-0.725 0.162-0.8 0.112-0.887 0.087-0.962 0.050-1.050 0.012-1.587-0.050-1.337-0.175-1.125-0.25-0.912-0.312-0.737-0.375-0.562-0.412-0.425-0.425-0.312-0.425-0.2-0.412-0.125-0.375-0.050-0.312-0.025-0.25 0.012-0.162v-6.673l0.063-0.8 0.162-0.675 0.262-0.575 0.325-0.475 0.375-0.4 0.412-0.3 0.437-0.25 0.437-0.175 0.412-0.125 0.375-0.075 0.325-0.050 0.262-0.025 0.162-0.012h7.298l0.862-0.062 0.737-0.175 0.625-0.262 0.512-0.35 0.412-0.4 0.337-0.437 0.25-0.45 0.187-0.45 0.125-0.437 0.087-0.4 0.050-0.35 0.025-0.262v-3.824h2.612l0.175 0.012zM10.939 3.704l-0.287 0.412-0.1 0.512 0.1 0.512 0.287 0.425 0.412 0.275 0.512 0.112 0.512-0.113 0.412-0.275 0.287-0.425 0.1-0.512-0.1-0.512-0.287-0.412-0.412-0.275-0.512-0.112-0.512 0.112zM18.812 1.229l1.125 0.25 0.912 0.325 0.737 0.375 0.562 0.4 0.425 0.425 0.312 0.425 0.2 0.412 0.125 0.375 0.050 0.325 0.025 0.25-0.012 0.162v6.673l-0.062 0.787-0.162 0.687-0.262 0.575-0.325 0.475-0.375 0.387-0.412 0.312-0.437 0.237-0.437 0.175-0.412 0.125-0.375 0.087-0.325 0.050-0.263 0.025h-7.46l-0.862 0.062-0.737 0.175-0.625 0.275-0.512 0.337-0.412 0.4-0.337 0.437-0.25 0.45-0.187 0.462-0.125 0.437-0.087 0.4-0.050 0.337-0.025 0.262v3.824h-2.787l-0.262-0.037-0.35-0.087-0.4-0.15-0.437-0.225-0.45-0.325-0.45-0.45-0.437-0.575-0.4-0.737-0.35-0.912-0.262-1.1-0.175-1.312-0.062-1.537 0.075-1.524 0.2-1.3 0.3-1.087 0.4-0.887 0.45-0.712 0.5-0.55 0.525-0.412 0.525-0.3 0.5-0.2 0.45-0.125 0.4-0.062 0.3-0.012h0.2l0.075 0.012h10.197v-1.037h-7.298l-0.012-3.436-0.025-0.462 0.062-0.425 0.137-0.387 0.212-0.35 0.312-0.325 0.387-0.287 0.475-0.25 0.55-0.225 0.637-0.187 0.725-0.15 0.8-0.125 0.887-0.075 0.962-0.050 1.050-0.025 1.587 0.063z"/></svg>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="var(--primary-3)" width=32 height=32 viewBox="0 0 24 24"><path d="M23.687 11.709l-.995-.616a13.559 13.559 0 0 0-.028-.29l.855-.797a.344.344 0 0 0-.114-.571l-1.093-.409a8.392 8.392 0 0 0-.086-.282l.682-.947a.342.342 0 0 0-.223-.538l-1.152-.187a9.243 9.243 0 0 0-.139-.259l.484-1.063c.05-.108.039-.234-.027-.332s-.178-.156-.297-.152l-1.17.041a6.662 6.662 0 0 0-.185-.224l.269-1.139a.343.343 0 0 0-.412-.412l-1.139.269a13.847 13.847 0 0 0-.225-.185l.041-1.17a.34.34 0 0 0-.484-.324l-1.063.485c-.086-.047-.172-.094-.259-.139l-.188-1.153a.344.344 0 0 0-.538-.223l-.948.682a8.383 8.383 0 0 0-.282-.085L14.562.596a.344.344 0 0 0-.571-.114l-.797.856a9.18 9.18 0 0 0-.29-.028l-.616-.995a.342.342 0 0 0-.582 0l-.616.995c-.097.008-.194.018-.29.028l-.798-.856a.342.342 0 0 0-.571.114l-.409 1.093c-.095.027-.188.056-.282.085l-.947-.682a.344.344 0 0 0-.538.223l-.188 1.153a9.243 9.243 0 0 0-.259.139l-1.063-.485a.342.342 0 0 0-.484.324l.041 1.17c-.076.06-.151.122-.225.185l-1.139-.269a.343.343 0 0 0-.412.412l.268 1.139c-.062.074-.124.149-.184.224l-1.17-.041a.342.342 0 0 0-.324.484l.485 1.063a9.055 9.055 0 0 0-.139.259l-1.152.187a.344.344 0 0 0-.223.538l.682.947c-.03.094-.058.187-.086.282L.59 9.435a.344.344 0 0 0-.114.571l.855.797a9.18 9.18 0 0 0-.028.29l-.995.616a.34.34 0 0 0 0 .582l.995.616c.008.097.018.194.028.29l-.855.798a.342.342 0 0 0 .114.571l1.093.409c.027.095.056.189.086.282l-.682.947a.341.341 0 0 0 .224.538l1.152.187c.045.087.091.173.139.259l-.485 1.063a.342.342 0 0 0 .324.484l1.169-.041c.061.076.122.151.185.225l-.268 1.14a.342.342 0 0 0 .412.411l1.139-.268c.074.063.149.124.225.184l-.041 1.17a.34.34 0 0 0 .484.323l1.063-.484c.086.048.172.094.259.139l.188 1.152a.344.344 0 0 0 .538.224l.947-.682c.094.03.187.059.282.086l.409 1.093a.341.341 0 0 0 .571.114l.798-.855c.096.011.193.02.29.029l.616.995a.343.343 0 0 0 .582 0l.616-.995c.097-.009.194-.018.29-.029l.797.855a.344.344 0 0 0 .571-.114l.409-1.093c.095-.027.189-.056.282-.086l.947.682a.341.341 0 0 0 .538-.224l.188-1.152c.087-.045.173-.092.259-.139l1.063.484a.343.343 0 0 0 .484-.323l-.041-1.17c.076-.06.151-.121.224-.184l1.139.268a.343.343 0 0 0 .412-.411l-.268-1.14c.062-.074.124-.149.184-.225l1.17.041a.34.34 0 0 0 .324-.484l-.484-1.063c.047-.086.094-.172.139-.259l1.152-.187a.344.344 0 0 0 .223-.538l-.682-.947.086-.282 1.093-.409a.342.342 0 0 0 .114-.571l-.855-.798c.01-.096.02-.193.028-.29l.995-.616a.34.34 0 0 0 0-.582zm-6.659 8.253a.705.705 0 0 1 .295-1.379.705.705 0 1 1-.296 1.379zm-.338-2.286a.641.641 0 0 0-.762.494l-.353 1.648c-1.09.495-2.3.77-3.575.77a8.63 8.63 0 0 1-3.65-.804l-.353-1.648a.64.64 0 0 0-.762-.493l-1.455.312a8.615 8.615 0 0 1-.752-.887h7.08c.08 0 .134-.014.134-.087v-2.505c0-.073-.053-.087-.134-.087h-2.071v-1.588h2.24c.204 0 1.093.058 1.377 1.194.089.349.284 1.486.418 1.85.133.408.675 1.223 1.253 1.223h3.528a.74.74 0 0 0 .128-.013 8.68 8.68 0 0 1-.802.941l-1.489-.32zm-9.793 2.252a.705.705 0 1 1-.296-1.379.705.705 0 0 1 .296 1.379zM4.211 9.036a.705.705 0 1 1-1.288.572.705.705 0 0 1 1.288-.572zm-.825 1.957l1.516-.674a.642.642 0 0 0 .326-.848l-.312-.706h1.228v5.534H3.667a8.668 8.668 0 0 1-.28-3.307zm6.652-.537V8.825h2.924c.151 0 1.066.175 1.066.859 0 .568-.702.772-1.279.772h-2.711zm10.626 1.468c0 .216-.008.431-.024.643h-.889c-.089 0-.125.058-.125.146v.408c0 .961-.542 1.17-1.017 1.223-.452.051-.953-.189-1.015-.466-.267-1.5-.711-1.821-1.413-2.374.871-.553 1.777-1.369 1.777-2.461 0-1.179-.809-1.922-1.36-2.287-.773-.51-1.629-.612-1.86-.612H5.545a8.658 8.658 0 0 1 4.847-2.736l1.084 1.137a.64.64 0 0 0 .907.021l1.212-1.16a8.668 8.668 0 0 1 5.931 4.224l-.83 1.875a.644.644 0 0 0 .326.848l1.598.71c.028.284.042.57.042.861zm-9.187-9.482a.703.703 0 1 1 .972 1.019.705.705 0 0 1-.972-1.019zm8.237 6.628c.157-.356.573-.516.928-.358a.705.705 0 1 1-.929.359z"/></svg>
                    <svg xmlns="http://www.w3.org/2000/svg" fill="var(--primary-3)" width=32 height=32 viewBox="0 0 27 26"><path d="m.98608 0h24.32332c.5446 0 .9861.436522.9861.975v24.05c0 .5385-.4415.975-.9861.975h-24.32332c-.544597 0-.98608-.4365-.98608-.975v-24.05c0-.538478.441483-.975.98608-.975zm13.63142 13.8324v-2.1324h-9.35841v2.1324h3.34111v9.4946h2.6598v-9.4946zm1.0604 9.2439c.4289.2162.9362.3784 1.5218.4865.5857.1081 1.2029.1622 1.8518.1622.6324 0 1.2331-.0595 1.8023-.1784.5691-.1189 1.0681-.3149 1.497-.5879s.7685-.6297 1.0187-1.0703.3753-.9852.3753-1.6339c0-.4703-.0715-.8824-.2145-1.2365-.1429-.3541-.3491-.669-.6186-.9447-.2694-.2757-.5925-.523-.9692-.7419s-.8014-.4257-1.2743-.6203c-.3465-.1406-.6572-.2771-.9321-.4095-.275-.1324-.5087-.2676-.7011-.4054-.1925-.1379-.3409-.2838-.4454-.4379-.1045-.154-.1567-.3284-.1567-.523 0-.1784.0467-.3392.1402-.4824.0935-.1433.2254-.2663.3959-.369s.3794-.1824.6269-.2392c.2474-.0567.5224-.0851.8248-.0851.22 0 .4523.0162.697.0486.2447.0325.4908.0825.7382.15.2475.0676.4881.1527.7218.2555.2337.1027.4495.2216.6475.3567v-2.4244c-.4015-.1514-.84-.2636-1.3157-.3365-.4756-.073-1.0214-.1095-1.6373-.1095-.6268 0-1.2207.0662-1.7816.1987-.5609.1324-1.0544.3392-1.4806.6203s-.763.6392-1.0104 1.0743c-.2475.4352-.3712.9555-.3712 1.5609 0 .7731.2268 1.4326.6805 1.9785.4537.546 1.1424 1.0082 2.0662 1.3866.363.146.7011.2892 1.0146.4298.3134.1405.5842.2865.8124.4378.2282.1514.4083.3162.5403.4946s.198.3811.198.6082c0 .1676-.0413.323-.1238.4662-.0825.1433-.2076.2676-.3753.373s-.3766.1879-.6268.2473c-.2502.0595-.5431.0892-.8785.0892-.5719 0-1.1383-.0986-1.6992-.2959-.5608-.1973-1.0805-.4933-1.5589-.8879z"></path></svg>
                </div>
            </div>
            <a class="github-link" href="https://github.com/zomatree" target="_blank" rel="nooopener noreferrer">github.com/Zomatree »</a>
        </div>
        <div class="down-arrow" on:click={scroll_to_blog}>
            <svg fill="#CAB8FD" height="20px" width="20px" version="1.1" id="Layer_1" xmlns="http://www.w3.org/2000/svg" xmlns:xlink="http://www.w3.org/1999/xlink" viewBox="0 0 330 330" xml:space="preserve"><g id="XMLID_169_"><path id="XMLID_197_" d="M304.394,139.394l-139.39,139.393L25.607,139.393c-5.857-5.857-15.355-5.858-21.213,0.001c-5.858,5.858-5.858,15.355,0,21.213l150.004,150c2.813,2.813,6.628,4.393,10.606,4.393s7.794-1.581,10.606-4.394l149.996-150c5.858-5.858,5.858-15.355,0-21.213C319.749,133.536,310.251,133.535,304.394,139.394z"/><path id="XMLID_221_" d="M154.398,190.607c2.813,2.813,6.628,4.393,10.606,4.393s7.794-1.581,10.606-4.394l149.996-150c5.858-5.858,5.858-15.355,0-21.213c-5.857-5.858-15.355-5.858-21.213,0l-139.39,139.393L25.607,19.393c-5.857-5.858-15.355-5.858-21.213,0c-5.858,5.858-5.858,15.355,0,21.213L154.398,190.607z"/></g></svg>
        </div>
    </div>
    <div class="section">
        <h1 class="header" id="about-me">
            About Me
        </h1>
        <p class="text about-me-content">
            I am a 18-year-old developer with a strong background in Rust, Python, and TypeScript. I have a passion for coding and am always looking to expand my skills and knowledge in the field. I am highly motivated and dedicated to creating efficient, high-quality code that is both functional and user-friendly. My experience in multiple programming languages allows me to approach problems from different angles and find the best solution for the task at hand. I am always eager to take on new challenges and am confident in my ability to excel in any project or task. I am excited to continue to grow and develop as a developer and look forward to working with others in the industry to create innovative and impactful technology.
        </p>
        <a href="/Resume.pdf" class="cv-link">
            [My Resume]
        </a>
    </div>
    <div id="bottom"/>
    <div class="section">
        <h1 class="header" id="projects">
            Projects
        </h1>
        <div class="grid">
            <ProjectCard
                title="Revolt"
                tag="Chat Platform"
                tag_colour="#ff4654"
                url="https://revolt.chat"
                github_url="https://github.com/revoltchat"
                description="Revolt is an open source user-first chat platform, built with Rust and Typescript it is an alternative to Discord."
            />
            <ProjectCard
                title="Revolt.py"
                tag="Python Library"
                tag_colour="#ffd342"
                github_url="https://github.com/revoltchat/revolt.py"
                description="API Wrapper written in Python for the Revolt API."
            />
            <ProjectCard
                title="Spec"
                tag="Python Library"
                tag_colour="#ffd342"
                github_url="https://github.com/Zomatree/Spec"
                description="Validation and serialization library for python which has support for advance features like tagged enums, renaming and custom validation."
            />
            <ProjectCard
                title="Zomatree.live"
                tag="Website"
                tag_colour="var(--primary-1)"
                url="https://zomatree.live"
                github_url="https://github.com/zomatree/zomatree.live"
                description="My personal website to show my projects and my blog."
            />
            <ProjectCard
                title="Kine"
                tag="Python Framework"
                tag_colour="#3673a5"
                github_url="https://github.com/zomatree/kine"
                description="React-like agnostic GUI framework for Python to make reactive websites and desktop apps."
            />
            <ProjectCard
                title="Homepage"
                tag="Website"
                tag_colour="#aad94c"
                github_url="https://github.com/zomatree/homepage"
                url="https://home.zomatree.live"
                description="Simple homepage with search and bookmarks for your browser's default page."
            />
        </div>
    </div>
    <div class="section">
        <h1 class="header" id="blogs">
            Blog Posts
        </h1>
        <div class="grid">
            <BlogCard
                title="Cake"
                href="/blog/cake"
                image={cake}
                alt="Slice of chocolate cake on white plate"
            />
            <BlogCard
                title="Cake"
                href="/blog/cake"
                image={cake}
                alt="Slice of chocolate cake on white plate"
            />
            <BlogCard
                title="Cake"
                href="/blog/cake"
                image={cake}
                alt="Slice of chocolate cake on white plate"
            />
            <BlogCard
                title="Cake"
                href="/blog/cake"
                image={cake}
                alt="Slice of chocolate cake on white plate"
            />
            <BlogCard
                title="Cake"
                href="/blog/cake"
                image={cake}
                alt="Slice of chocolate cake on white plate"
            />
        </div>
    </div>
</div>
