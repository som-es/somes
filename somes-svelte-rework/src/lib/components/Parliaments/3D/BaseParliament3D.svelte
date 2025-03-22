<script lang="ts">
	import type { Bubble } from '$lib/parliament';

	import { interactivity } from '@threlte/extras';
	interactivity();

	export let circles2d: Bubble[][];
	export let selected: Bubble | null;
	export let preview: boolean = false;
	export let select: (bubble: Bubble, event: MouseEvent | KeyboardEvent | null) => void;

	import { T, useFrame } from '@threlte/core';
	import { ContactShadows, CSM, OrbitControls, useTexture } from '@threlte/extras';
	import {
		BoxGeometry,
		ImageUtils,
		MeshBasicMaterial,
		MeshPhongMaterial,
		MeshStandardMaterial
	} from 'three';
	import { generateHalfCircle, setupParliament } from '$lib/parliament';
	import Angle20 from './Angle20.svelte';
	import { address } from '$lib/api/api';

	let lightAngle = Math.PI;
	let lightRadius = 50;
	let directionalLightX = 0;
	let directionalLightZ = 0;
	let direction = 1;

	useFrame((ctx, delta) => {
		if (lightAngle < Math.PI) {
			direction = 1;
		}
		if (lightAngle > Math.PI * 2) {
			direction = -1;
		}

		lightAngle += 1.5 * delta * direction;

		directionalLightX = Math.cos(lightAngle) * lightRadius;
		directionalLightZ = Math.sin(lightAngle) * lightRadius;
	});

	$: if (circles2d) {
		circles2d = structuredClone(circles2d);
		const useOffset = true;
		const startingRadius = 13;

		// lifecycle zeug - deshalb hier
		//   circles2d[del.seat_row - 1] [del.seat_col - 1].texture = useTexture("http://localhost:3000/assets/" + del.id + ".jpg");
		// circles2d[del.seat_row - 1] [del.seat_col - 1].material = partyToStandardMaterial.get(del.party) ?? null;
		circles2d.forEach((circles, idx) => {
			if (idx == circles2d.length - 1) {
				return;
			}
			const halfCircle = generateHalfCircle(
				circles.length,
				startingRadius + (useOffset ? idx * (idx == 1 ? 7 : 3) + (idx >= 2 ? 7 : 0) : idx * 1),
				0,
				0
			);

			circles.forEach((circle, i) => {
				circle.x = halfCircle[halfCircle.length - i - 1].x;
				circle.y = halfCircle[halfCircle.length - i - 1].y;
				circle.angle_rad = Math.PI - halfCircle[i].angle_rad;

				if (circle.color && circle.del) {
					circle.texture = useTexture(address + '/assets/' + circle.del.id + '.jpg');
					circle.material = new MeshStandardMaterial({
						color: circle.color ?? '#FFFFFF',
						blending: 1
					});
				}
			});
		});
		// const circle = circles2d[del.seat_row - 1][del.seat_col - 1];
	}

	const selectedMaterial = new MeshStandardMaterial({ color: 'orange', blending: 1 });
</script>

<!-- <CSM
  enabled={true}
  lightDirection={[directionalLightX, -1, directionalLightZ]}
  lightIntensity={1.0}
  lightColor={0xd7681c}> -->
{#each circles2d as halfCircle, i}
	{#each halfCircle as circle}
		{#if circle.material && circle.del}
			<Angle20
				position={[circle.y, i + 1, circle.x]}
				rotation.y={circle.angle_rad}
				material={selected?.del?.id == circle.del?.id ? selectedMaterial : circle.material}
				on:click={(e) => {
					select(circle, null);
				}}
			/>

			{#await circle.texture then value}
				<T.Mesh
					position={[
						circle.y + Math.sin(circle.angle_rad) * 1.3,
						i + 3.5,
						circle.x + Math.cos(circle.angle_rad) * 1.3
					]}
				>
					<T.SphereGeometry />
					<T.MeshStandardMaterial map={value} />
				</T.Mesh>
			{/await}
		{/if}
	{/each}
{/each}

<T.PerspectiveCamera makeDefault position={[-70, 10, 10]} rotation.y={10} fov={45}>
	<OrbitControls enableZoom={true} enableDamping autoRotateSpeed={0.5} target.y={1.5} />
</T.PerspectiveCamera>

<T.PointLight
	color={0xd7681c}
	intensity={1.0}
	position.z={directionalLightX}
	position.y={20}
	position.x={directionalLightZ}
	power={25434}
/>
<T.PointLight
	color={0xd7681c}
	intensity={1.0}
	position.z={-10}
	position.y={30}
	position.x={-30}
	power={91434}
/>

<T.PointLight
	color={0xd7681c}
	intensity={1.0}
	position.z={60}
	position.y={91}
	position.x={40}
	power={91434}
/>

<T.AmbientLight intensity={0.5} />
<!-- <Grid
  position.y={-0.001}
  cellColor="#ffffff"
  sectionColor="#ffffff"
  sectionThickness={0}
  fadeDistance={250}
  cellSize={2}
/> -->

<ContactShadows scale={10} blur={2} far={2.5} opacity={0.5} />

<T.Mesh position={[0, -0.001, 0]} geometry={new BoxGeometry(350, 1, 350)} receiveShadow>
	<T.MeshStandardMaterial color="#FFFFFF" />
</T.Mesh>

<!-- </CSM> -->
