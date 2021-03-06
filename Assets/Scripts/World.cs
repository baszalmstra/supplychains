﻿using UnityEngine;
using System.Collections;
using System.Collections.Generic;

public class World : MonoBehaviour
{
    public WorldBlocks Blocks { get; private set; }
    public WorldChunks Chunks { get; private set; }
    public WorldGenerator Generator { get; private set; }

    private Dictionary<BlockPos, GameObject> chunkObjects;

    public World()
    {
        Blocks = new WorldBlocks(this);
        Chunks = new WorldChunks(this);
        Generator = new WorldGenerator();
        chunkObjects = new Dictionary<BlockPos, GameObject>();
    }

    public Material ChunkMaterial;

    public void Start()
    {
        int size = 20;
        for(int cx = -size; cx < size; ++cx)
            for(int cy = -size; cy < size; ++cy)
            {
                var pos = new BlockPos(cx*Constants.ChunkSize, 0, cy*Constants.ChunkSize);
                Chunk chunk = Generator.Generate(this, pos);
                Chunks.Set(chunk.Position, chunk);
            }

        SpawnChunks();
    }

    void SpawnChunks()
    {
        int size = 20;
        for(int cx = -size; cx < size; ++cx)
            for(int cy = -size; cy < size; ++cy)
            {
                var chunk = Chunks.Get(new BlockPos(cx*Constants.ChunkSize, 0, cy*Constants.ChunkSize));
                if(chunk != null) 
                    SpawnChunkGameObject(chunk);
            }
    }
    
    GameObject SpawnChunkGameObject(Chunk chunk)
    {
        GameObject obj = new GameObject();
        obj.name = "Chunk " + chunk.Position.ToString();
        obj.AddComponent<MeshFilter>().mesh = ChunkMeshBuilder.Build(chunk);
        obj.AddComponent<MeshRenderer>().material = ChunkMaterial;
        obj.transform.localPosition = new Vector3(
            chunk.Position.x * Constants.BlockSize.x,
            chunk.Position.y * Constants.BlockSize.y,
            chunk.Position.z * Constants.BlockSize.z);
        obj.transform.SetParent(this.transform);

        chunkObjects.Add(chunk.Position, obj);

        return obj;
    }
}
