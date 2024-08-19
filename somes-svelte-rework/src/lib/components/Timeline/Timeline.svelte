<script lang="ts">
</script>

<section class="cd-h-timeline js-cd-h-timeline margin-bottom-md">
	<div class="cd-h-timeline__container container">
		<div class="cd-h-timeline__dates">
			<div class="cd-h-timeline__line">
				<ol>
					<li>
						<a
							href="#0"
							data-date="16/01/2014"
							class="cd-h-timeline__date cd-h-timeline__date--selected">16 Jan</a
						>
					</li>
					<li><a href="#0" data-date="28/02/2014" class="cd-h-timeline__date">28 Feb</a></li>
					<!-- other dates here -->
				</ol>

				<span class="cd-h-timeline__filling-line" aria-hidden="true"></span>
			</div>
			<!-- .cd-h-timeline__line -->
		</div>
		<!-- .cd-h-timeline__dates -->

		<ul>
			<li>
				<a
					href="#0"
					class="text-replace cd-h-timeline__navigation cd-h-timeline__navigation--prev cd-h-timeline__navigation--inactive"
					>Prev</a
				>
			</li>
			<li>
				<a href="#0" class="text-replace cd-h-timeline__navigation cd-h-timeline__navigation--next"
					>Next</a
				>
			</li>
		</ul>
	</div>
	<!-- .cd-h-timeline__container -->

	<div class="cd-h-timeline__events">
		<ol>
			<li class="cd-h-timeline__event cd-h-timeline__event--selected text-component">
				<div class="cd-h-timeline__event-content container">
					<h2 class="cd-h-timeline__event-title">Horizontal Timeline</h2>
					<em class="cd-h-timeline__event-date">January 16th, 2014</em>
					<p class="cd-h-timeline__event-description color-contrast-medium">
						Lorem ipsum dolor sit amet, consectetur adipisicing elit. Illum praesentium officia,
						fugit recusandae ipsa, quia velit nulla adipisci? Consequuntur aspernatur at, eaque hic
						repellendus sit dicta consequatur quae, ut harum ipsam molestias maxime non nisi
						reiciendis eligendi! Doloremque quia pariatur harum ea amet quibusdam quisquam, quae,
						temporibus dolores porro doloribus.
					</p>
				</div>
			</li>

			<li class="cd-h-timeline__event text-component">
				<!-- event description here -->
			</li>

			<!-- other descriptions here -->
		</ol>
	</div>
	<!-- .cd-h-timeline__events -->
</section>

<style>
	.cd-h-timeline__events {
		position: relative;
	}

	.cd-h-timeline__event {
		position: absolute;
		z-index: 1;
		width: 100%;
		left: 0;
		top: 0;
		transform: translateX(-100%);
		opacity: 0;
		animation-duration: 0.4s;
		animation-timing-function: ease-in-out;
	}

	.cd-h-timeline__event--selected {
		position: relative;
		z-index: 2;
		opacity: 1;
		transform: translateX(0);
	}

	.cd-h-timeline__event--enter-right,
	.cd-h-timeline__event--leave-right {
		animation-name: cd-enter-right;
	}

	.cd-h-timeline__event--enter-left,
	.cd-h-timeline__event--leave-left {
		animation-name: cd-enter-left;
	}

	.cd-h-timeline__event--leave-right,
	.cd-h-timeline__event--leave-left {
		animation-direction: reverse;
	}

	@keyframes cd-enter-right {
		0% {
			opacity: 0;
			transform: translateX(100%);
		}

		100% {
			opacity: 1;
			transform: translateX(0%);
		}
	}

	@keyframes cd-enter-left {
		0% {
			opacity: 0;
			transform: translateX(-100%);
		}

		100% {
			opacity: 1;
			transform: translateX(0%);
		}
	}
</style>
