import bpy
import bpy_extras
import bmesh
import time

import re
from pprint import pprint
from array import *




def renderDroplet(filename,objectname):

    global n,center
    faces = [(0, 1, 2, 3),
             (4, 7, 6, 5),
             (0, 4, 5, 1),
             (1, 5, 6, 2),
             (2, 6, 7, 3),
             (4, 0, 3, 7),
            ]
    mesh = bpy.data.meshes.new(objectname)
    bm = bmesh.new()
    
    dim=0.5

    with open(filename) as f:
        line = f.readline().strip()
        while line:
            print(line)
            (a,b,c) = line.split(',')
            x=int(a)
            y=int(b)
            z=int(c)
    
            print("Adding Box at:",x,y,z)
            verts_loc = [(x+dim, y+dim, z-dim),
                            (x+dim, y-dim, z-dim),
                            (x-dim, y-dim, z-dim),
                            (x-dim, y+dim, z-dim),
                            (x+dim, y+dim, z+dim),
                            (x+dim, y-dim, z+dim),
                            (x-dim, y-dim, z+dim),
                            (x-dim, y+dim, z+dim),
                            ]
            f_offset = len(bm.verts)
            for v_co in verts_loc:
                bm.verts.new(v_co)

            for f_idx in faces:
                bm.verts.ensure_lookup_table()
                bm.faces.new([bm.verts[i+f_offset] for i in f_idx])
            line = f.readline().strip()

    bm.to_mesh(mesh)
    mesh.update()

    bpy_extras.object_utils.object_data_add(bpy.context, mesh)
    bpy.data.objects[name].location=[-center,-center,0]
    mat = bpy.data.materials.new(name=name)
    bpy.data.objects[name].data.materials.append(mat)
    # Probably do this outside the loop...
    bpy.ops.object.select_by_type(type='MESH')
    bpy.ops.object.origin_set(type='ORIGIN_CENTER_OF_VOLUME', center='MEDIAN')

    

#renderDroplet('input_sample.txt','sample');
renderDroplet('input.txt','actual');

print("DONE")
