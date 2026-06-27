<script lang="ts">
    import Carousel from "$lib/components/Carousel.svelte";

</script>
<div class="card bg-neutral w-96 shadow-xl">
    <div class="card-body text-primary">
        <div class="flex flex-row space-x-2 font-mono">
            <span class="badge badge-outline badge-sm badge-secondary">SOCIALS</span>
            <span class="badge badge-outline badge-sm badge-secondary">ATTRACTION</span>
        </div>
        <Carousel autoScroll={true} interval={8000}>
            <div class="carousel-item w-full">
                <div class="stat">
                    <div class="stat-figure">
                    <div class="avatar avatar-online">
                        <div class="w-16 rounded-full">
                            <img alt="anakeen" src="https://img.daisyui.com/images/profile/demo/anakeen@192.webp" />
                        </div>
                    </div>
                    </div>
                    <div class="stat-value">89,400</div>
                    <div class="stat-title text-primary">Total Followers</div>
                    <div class="stat-desc text-success">21% more than last month</div>
                </div>
            </div>
            <div class="carousel-item w-full">
                <div class="stat">
                    <div class="stat-figure">
                    <div class="avatar avatar-online">
                        <div class="w-16 rounded-full">
                            <img alt="anakeen" src="https://img.daisyui.com/images/profile/demo/anakeen@192.webp" />
                        </div>
                    </div>
                    </div>
                    <div class="stat-value">18</div>
                    <div class="stat-title text-primary">Posts Today Cross-Platform</div>
                    <div class="stat-desc text-success">Keep it up!</div>
                </div>
            </div>
            <div class="carousel-item w-full">
                <div class="stat">
                    <div class="stat-figure">
                    <div class="avatar avatar-online">
                        <div class="w-16 rounded-full">
                            <img alt="anakeen" src="https://img.daisyui.com/images/profile/demo/anakeen@192.webp" />
                        </div>
                    </div>
                    </div>
                    <div class="stat-value">265K</div>
                    <div class="stat-title text-primary">Total Engagements</div>
                    <div class="stat-desc text-success">83% more than last week</div>
                </div>
            </div>
        </Carousel>
    </div>
</div>